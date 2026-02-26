package main

import (
	"bytes"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

type pageRange struct {
	name  string
	start int
	end   int
}

type bookmark struct {
	title string
	page  int
}

type alignmentCheck struct {
	rangeName string
	pattern   string
}

var printedRanges = []pageRange{
	{"01-introduction", 13, 23},
	{"02-system-bus", 24, 34},
	{"03a-processor-SIO", 35, 81},
	{"03b-processor-interrupts-debug", 82, 99},
	{"03c-cortex-m33-coprocessors", 100, 122},
	{"03d-cortex-m33-processor", 123, 232},
	{"03e-hazard3-processor", 233, 334},
	{"03f-architecture-switching", 335, 336},
	{"04-memory", 337, 352},
	{"05a-bootrom-concepts", 353, 374},
	{"05b-bootrom-apis", 375, 415},
	{"05c-bootrom-usb-uart", 416, 440},
	{"06-power", 441, 493},
	{"07-resets", 494, 512},
	{"08a-clocks-overview", 513, 553},
	{"08b-clocks-oscillators", 554, 586},
	{"09a-gpio-overview", 587, 603},
	{"09b-gpio-io-user-bank", 604, 759},
	{"09c-gpio-io-qspi-pads", 760, 815},
	{"10-security", 816, 875},
	{"11a-pio-overview-model", 876, 888},
	{"11b-pio-instructions", 889, 901},
	{"11c-pio-details-examples", 902, 960},
	{"12a-uart", 961, 982},
	{"12b-i2c", 983, 1045},
	{"12c-spi", 1046, 1065},
	{"12d-adc-temp", 1066, 1075},
	{"12e-pwm", 1076, 1093},
	{"12f-dma", 1094, 1140},
	{"12g-usb", 1141, 1181},
	{"12h-timers-watchdog", 1182, 1201},
	{"12i-hstx", 1202, 1211},
	{"12j-trng-sha256", 1212, 1225},
	{"12k-qspi-qmi", 1226, 1248},
	{"12l-system-control", 1249, 1267},
	{"13-otp", 1268, 1326},
	{"14-electrical", 1327, 1348},
	{"15-appendices", 1349, 1378},
}

var bookmarkChecks = []alignmentCheck{
	{"01-introduction", `^Chapter 1[.] `},
	{"02-system-bus", `^Chapter 2[.] `},
	{"03a-processor-SIO", `^Chapter 3[.] `},
	{"04-memory", `^Chapter 4[.] `},
	{"05a-bootrom-concepts", `^Chapter 5[.] `},
	{"06-power", `^Chapter 6[.] `},
	{"07-resets", `^Chapter 7[.] `},
	{"08a-clocks-overview", `^Chapter 8[.] `},
	{"09a-gpio-overview", `^Chapter 9[.] `},
	{"10-security", `^Chapter 10[.] `},
	{"11a-pio-overview-model", `^Chapter 11[.] `},
	{"12a-uart", `^Chapter 12[.] `},
	{"13-otp", `^Chapter 13[.] `},
	{"14-electrical", `^Chapter 14[.] `},
	{"15-appendices", `^Appendix A:`},
}

func die(format string, args ...any) {
	fmt.Fprintf(os.Stderr, "error: "+format+"\n", args...)
	os.Exit(1)
}

func warn(format string, args ...any) {
	fmt.Fprintf(os.Stderr, "warning: "+format+"\n", args...)
}

func requireCommands(names ...string) {
	for _, name := range names {
		if _, err := exec.LookPath(name); err != nil {
			die("%s is required but not installed", name)
		}
	}
}

func runCapture(command string, args ...string) string {
	cmd := exec.Command(command, args...)
	var stderr bytes.Buffer
	cmd.Stderr = &stderr
	output, err := cmd.Output()
	if err != nil {
		details := strings.TrimSpace(stderr.String())
		if details == "" {
			details = err.Error()
		}
		die("command failed: %s %s (%s)", command, strings.Join(args, " "), details)
	}
	return string(output)
}

func run(command string, args ...string) {
	cmd := exec.Command(command, args...)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		die("command failed: %s %s (%v)", command, strings.Join(args, " "), err)
	}
}

func parseBookmarks(dump string) []bookmark {
	var bookmarks []bookmark
	currentTitle := ""

	for _, line := range strings.Split(dump, "\n") {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "BookmarkTitle: ") {
			currentTitle = strings.TrimPrefix(trimmed, "BookmarkTitle: ")
			continue
		}

		if strings.HasPrefix(trimmed, "BookmarkPageNumber: ") && currentTitle != "" {
			pageText := strings.TrimPrefix(trimmed, "BookmarkPageNumber: ")
			pageNumber, err := strconv.Atoi(pageText)
			if err != nil {
				die("invalid bookmark page number: %s", pageText)
			}
			bookmarks = append(bookmarks, bookmark{title: currentTitle, page: pageNumber})
			currentTitle = ""
		}
	}

	if len(bookmarks) == 0 {
		die("no bookmarks found in PDF metadata")
	}

	return bookmarks
}

func bookmarkPage(bookmarks []bookmark, pattern string) int {
	compiled := regexp.MustCompile(pattern)
	for _, item := range bookmarks {
		if compiled.MatchString(item.title) {
			return item.page
		}
	}
	die("could not find bookmark matching regex: %s", pattern)
	return 0
}

func rangeStart(ranges []pageRange, name string) (int, bool) {
	for _, item := range ranges {
		if item.name == name {
			return item.start, true
		}
	}
	return 0, false
}

func buildPhysicalRanges(bookmarks []bookmark) ([]pageRange, int) {
	chapter1Start := bookmarkPage(bookmarks, `^Chapter 1[.] `)
	appendixStart := bookmarkPage(bookmarks, `^Appendix A: `)

	pageOffset := chapter1Start - 13
	if pageOffset < 0 {
		die("calculated negative page offset: %d", pageOffset)
	}

	frontMatterEnd := chapter1Start - 1
	if frontMatterEnd < 1 {
		die("invalid front matter end page: %d", frontMatterEnd)
	}

	physicalRanges := []pageRange{{name: "00-front-matter", start: 1, end: frontMatterEnd}}
	for _, item := range printedRanges {
		physicalRanges = append(physicalRanges, pageRange{
			name:  item.name,
			start: item.start + pageOffset,
			end:   item.end + pageOffset,
		})
	}

	appendicesRangeStart, found := rangeStart(physicalRanges, "15-appendices")
	if !found {
		die("missing generated range for 15-appendices")
	}
	if appendicesRangeStart != appendixStart {
		die(
			"appendix start mismatch: range starts at %d, bookmark starts at %d",
			appendicesRangeStart,
			appendixStart,
		)
	}

	return physicalRanges, pageOffset
}

func validateRanges(ranges []pageRange, totalPages int) {
	previousEnd := 0
	for lineNumber, item := range ranges {
		displayLine := lineNumber + 1
		if item.start > item.end {
			die("line %d has start > end: %s %d-%d", displayLine, item.name, item.start, item.end)
		}
		if item.start <= previousEnd {
			die(
				"line %d overlaps prior range: %s starts at %d, previous ended at %d",
				displayLine, item.name, item.start, previousEnd,
			)
		}
		if item.start != previousEnd+1 {
			die(
				"line %d has gap before %s: expected %d, got %d",
				displayLine, item.name, previousEnd+1, item.start,
			)
		}
		if item.end > totalPages {
			die(
				"line %d exceeds source page count (%d): %s ends at %d",
				displayLine, totalPages, item.name, item.end,
			)
		}
		previousEnd = item.end
	}

	if previousEnd < totalPages {
		warn(
			"ranges end at %d but source has %d pages (trailing pages excluded)",
			previousEnd, totalPages,
		)
	}
}

func validateBookmarkAlignment(ranges []pageRange, bookmarks []bookmark) {
	for _, check := range bookmarkChecks {
		expected := bookmarkPage(bookmarks, check.pattern)
		actual, found := rangeStart(ranges, check.rangeName)
		if !found {
			die("could not find generated range: %s", check.rangeName)
		}
		if actual != expected {
			die(
				"bookmark mismatch for %s: range starts at %d, bookmark starts at %d",
				check.rangeName, actual, expected,
			)
		}
	}
}

func splitRange(srcPath, outDir string, item pageRange) {
	pages := item.end - item.start + 1
	fmt.Printf("  %s (%dpp: %d-%d)\n", item.name, pages, item.start, item.end)
	outputPath := filepath.Join(outDir, item.name+".pdf")
	run(
		"qpdf",
		srcPath,
		"--pages",
		".",
		fmt.Sprintf("%d-%d", item.start, item.end),
		"--",
		outputPath,
	)
}

func humanSize(sizeBytes int64) string {
	units := []string{"B", "K", "M", "G", "T"}
	size := float64(sizeBytes)
	unitIndex := 0
	for size >= 1024 && unitIndex < len(units)-1 {
		size /= 1024
		unitIndex++
	}
	if unitIndex == 0 {
		return fmt.Sprintf("%d%s", int(size), units[unitIndex])
	}
	return fmt.Sprintf("%.1f%s", size, units[unitIndex])
}

func totalPages(srcPath string) int {
	text := strings.TrimSpace(runCapture("qpdf", "--show-npages", srcPath))
	pages, err := strconv.Atoi(text)
	if err != nil {
		die("invalid total page count from qpdf: %s", text)
	}
	return pages
}

func defaultBaseDir() string {
	candidates := []string{}

	executablePath, execErr := os.Executable()
	if execErr == nil {
		candidates = append(candidates, filepath.Dir(executablePath))
	}
	currentDir, cwdErr := os.Getwd()
	if cwdErr == nil {
		candidates = append(candidates, currentDir, filepath.Join(currentDir, "rp2350-reference"))
	}

	for _, candidate := range candidates {
		pdfPath := filepath.Join(candidate, "rp2350-datasheet.pdf")
		if _, err := os.Stat(pdfPath); err == nil {
			return candidate
		}
	}

	if len(candidates) > 0 {
		return candidates[0]
	}
	return "."
}

func listAndPrintOutputPDFs(outDir string) {
	entries := []struct {
		name string
		size int64
	}{}

	filePattern := filepath.Join(outDir, "*.pdf")
	matches, globErr := filepath.Glob(filePattern)
	if globErr != nil {
		die("failed to list output PDFs: %v", globErr)
	}
	for _, match := range matches {
		info, statErr := os.Stat(match)
		if statErr != nil {
			if errors.Is(statErr, fs.ErrNotExist) {
				continue
			}
			die("failed to stat output PDF %s: %v", match, statErr)
		}
		entries = append(entries, struct {
			name string
			size int64
		}{
			name: filepath.Base(match),
			size: info.Size(),
		})
	}

	sort.Slice(entries, func(first, second int) bool {
		return entries[first].size > entries[second].size
	})

	for _, entry := range entries {
		fmt.Printf("%5s %s\n", humanSize(entry.size), entry.name)
	}
}

func main() {
	if len(os.Args) > 3 {
		fmt.Fprintf(os.Stderr, "usage: %s [source.pdf] [output_dir]\n", filepath.Base(os.Args[0]))
		os.Exit(2)
	}

	baseDir := defaultBaseDir()
	srcPath := filepath.Join(baseDir, "rp2350-datasheet.pdf")
	outDir := filepath.Join(baseDir, "datasheet")

	if len(os.Args) >= 2 {
		srcPath = os.Args[1]
	}
	if len(os.Args) >= 3 {
		outDir = os.Args[2]
	}

	requireCommands("qpdf", "pdftk")
	if _, err := os.Stat(srcPath); err != nil {
		die("source PDF not found: %s", srcPath)
	}

	bookmarkDump := runCapture("pdftk", srcPath, "dump_data")
	bookmarks := parseBookmarks(bookmarkDump)
	ranges, pageOffset := buildPhysicalRanges(bookmarks)
	validateRanges(ranges, totalPages(srcPath))
	validateBookmarkAlignment(ranges, bookmarks)

	if err := os.MkdirAll(outDir, 0o755); err != nil {
		die("failed to create output directory %s: %v", outDir, err)
	}

	fmt.Printf("Splitting RP2350 datasheet from: %s (page offset: +%d)\n\n", srcPath, pageOffset)
	for _, item := range ranges {
		splitRange(srcPath, outDir, item)
	}

	fmt.Printf("\nDone! Files in: %s\n", outDir)
	listAndPrintOutputPDFs(outDir)
}
