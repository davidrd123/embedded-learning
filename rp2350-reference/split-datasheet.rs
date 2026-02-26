use std::cmp::Reverse;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Clone, Debug)]
struct PageRange {
    name: &'static str,
    start: i32,
    end: i32,
}

#[derive(Clone, Debug)]
struct Bookmark {
    title: String,
    page: i32,
}

#[derive(Clone, Debug)]
struct BookmarkCheck {
    range_name: &'static str,
    title_prefix: &'static str,
}

fn printed_ranges() -> Vec<PageRange> {
    vec![
        PageRange {
            name: "01-introduction",
            start: 13,
            end: 23,
        },
        PageRange {
            name: "02-system-bus",
            start: 24,
            end: 34,
        },
        PageRange {
            name: "03a-processor-SIO",
            start: 35,
            end: 81,
        },
        PageRange {
            name: "03b-processor-interrupts-debug",
            start: 82,
            end: 99,
        },
        PageRange {
            name: "03c-cortex-m33-coprocessors",
            start: 100,
            end: 122,
        },
        PageRange {
            name: "03d-cortex-m33-processor",
            start: 123,
            end: 232,
        },
        PageRange {
            name: "03e-hazard3-processor",
            start: 233,
            end: 334,
        },
        PageRange {
            name: "03f-architecture-switching",
            start: 335,
            end: 336,
        },
        PageRange {
            name: "04-memory",
            start: 337,
            end: 352,
        },
        PageRange {
            name: "05a-bootrom-concepts",
            start: 353,
            end: 374,
        },
        PageRange {
            name: "05b-bootrom-apis",
            start: 375,
            end: 415,
        },
        PageRange {
            name: "05c-bootrom-usb-uart",
            start: 416,
            end: 440,
        },
        PageRange {
            name: "06-power",
            start: 441,
            end: 493,
        },
        PageRange {
            name: "07-resets",
            start: 494,
            end: 512,
        },
        PageRange {
            name: "08a-clocks-overview",
            start: 513,
            end: 553,
        },
        PageRange {
            name: "08b-clocks-oscillators",
            start: 554,
            end: 586,
        },
        PageRange {
            name: "09a-gpio-overview",
            start: 587,
            end: 603,
        },
        PageRange {
            name: "09b-gpio-io-user-bank",
            start: 604,
            end: 759,
        },
        PageRange {
            name: "09c-gpio-io-qspi-pads",
            start: 760,
            end: 815,
        },
        PageRange {
            name: "10-security",
            start: 816,
            end: 875,
        },
        PageRange {
            name: "11a-pio-overview-model",
            start: 876,
            end: 888,
        },
        PageRange {
            name: "11b-pio-instructions",
            start: 889,
            end: 901,
        },
        PageRange {
            name: "11c-pio-details-examples",
            start: 902,
            end: 960,
        },
        PageRange {
            name: "12a-uart",
            start: 961,
            end: 982,
        },
        PageRange {
            name: "12b-i2c",
            start: 983,
            end: 1045,
        },
        PageRange {
            name: "12c-spi",
            start: 1046,
            end: 1065,
        },
        PageRange {
            name: "12d-adc-temp",
            start: 1066,
            end: 1075,
        },
        PageRange {
            name: "12e-pwm",
            start: 1076,
            end: 1093,
        },
        PageRange {
            name: "12f-dma",
            start: 1094,
            end: 1140,
        },
        PageRange {
            name: "12g-usb",
            start: 1141,
            end: 1181,
        },
        PageRange {
            name: "12h-timers-watchdog",
            start: 1182,
            end: 1201,
        },
        PageRange {
            name: "12i-hstx",
            start: 1202,
            end: 1211,
        },
        PageRange {
            name: "12j-trng-sha256",
            start: 1212,
            end: 1225,
        },
        PageRange {
            name: "12k-qspi-qmi",
            start: 1226,
            end: 1248,
        },
        PageRange {
            name: "12l-system-control",
            start: 1249,
            end: 1267,
        },
        PageRange {
            name: "13-otp",
            start: 1268,
            end: 1326,
        },
        PageRange {
            name: "14-electrical",
            start: 1327,
            end: 1348,
        },
        PageRange {
            name: "15-appendices",
            start: 1349,
            end: 1378,
        },
    ]
}

fn bookmark_checks() -> Vec<BookmarkCheck> {
    vec![
        BookmarkCheck {
            range_name: "01-introduction",
            title_prefix: "Chapter 1. ",
        },
        BookmarkCheck {
            range_name: "02-system-bus",
            title_prefix: "Chapter 2. ",
        },
        BookmarkCheck {
            range_name: "03a-processor-SIO",
            title_prefix: "Chapter 3. ",
        },
        BookmarkCheck {
            range_name: "04-memory",
            title_prefix: "Chapter 4. ",
        },
        BookmarkCheck {
            range_name: "05a-bootrom-concepts",
            title_prefix: "Chapter 5. ",
        },
        BookmarkCheck {
            range_name: "06-power",
            title_prefix: "Chapter 6. ",
        },
        BookmarkCheck {
            range_name: "07-resets",
            title_prefix: "Chapter 7. ",
        },
        BookmarkCheck {
            range_name: "08a-clocks-overview",
            title_prefix: "Chapter 8. ",
        },
        BookmarkCheck {
            range_name: "09a-gpio-overview",
            title_prefix: "Chapter 9. ",
        },
        BookmarkCheck {
            range_name: "10-security",
            title_prefix: "Chapter 10. ",
        },
        BookmarkCheck {
            range_name: "11a-pio-overview-model",
            title_prefix: "Chapter 11. ",
        },
        BookmarkCheck {
            range_name: "12a-uart",
            title_prefix: "Chapter 12. ",
        },
        BookmarkCheck {
            range_name: "13-otp",
            title_prefix: "Chapter 13. ",
        },
        BookmarkCheck {
            range_name: "14-electrical",
            title_prefix: "Chapter 14. ",
        },
        BookmarkCheck {
            range_name: "15-appendices",
            title_prefix: "Appendix A:",
        },
    ]
}

fn die(message: &str) -> ! {
    eprintln!("error: {message}");
    std::process::exit(1);
}

fn warn(message: &str) {
    eprintln!("warning: {message}");
}

fn require_commands(commands: &[&str]) {
    for command in commands {
        if Command::new(command)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_err()
        {
            die(&format!("{command} is required but not installed"));
        }
    }
}

fn run_capture(command: &str, args: &[&str]) -> String {
    let output = Command::new(command).args(args).output().unwrap_or_else(|error| {
        die(&format!(
            "failed to execute command: {command} {} ({error})",
            args.join(" ")
        ));
    });
    if !output.status.success() {
        let stderr_text = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let details = if stderr_text.is_empty() {
            format!("exit {}", output.status)
        } else {
            stderr_text
        };
        die(&format!(
            "command failed: {command} {} ({details})",
            args.join(" ")
        ));
    }
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn run(command: &str, args: &[&str]) {
    let status = Command::new(command)
        .args(args)
        .status()
        .unwrap_or_else(|error| {
            die(&format!(
                "failed to execute command: {command} {} ({error})",
                args.join(" ")
            ));
        });
    if !status.success() {
        die(&format!(
            "command failed: {command} {} (exit {status})",
            args.join(" ")
        ));
    }
}

fn parse_bookmarks(bookmark_dump: &str) -> Vec<Bookmark> {
    let mut current_title: Option<String> = None;
    let mut bookmarks: Vec<Bookmark> = Vec::new();

    for line in bookmark_dump.lines() {
        let trimmed = line.trim();
        if let Some(title) = trimmed.strip_prefix("BookmarkTitle: ") {
            current_title = Some(title.to_string());
            continue;
        }

        if let Some(page_text) = trimmed.strip_prefix("BookmarkPageNumber: ") {
            if let Some(title) = current_title.take() {
                let page = page_text.parse::<i32>().unwrap_or_else(|_| {
                    die(&format!("invalid bookmark page number: {page_text}"));
                });
                bookmarks.push(Bookmark { title, page });
            }
        }
    }

    if bookmarks.is_empty() {
        die("no bookmarks found in PDF metadata");
    }

    bookmarks
}

fn bookmark_page(bookmarks: &[Bookmark], prefix: &str) -> i32 {
    for bookmark in bookmarks {
        if bookmark.title.starts_with(prefix) {
            return bookmark.page;
        }
    }
    die(&format!(
        "could not find bookmark starting with: {prefix}"
    ));
}

fn range_start(ranges: &[PageRange], range_name: &str) -> Option<i32> {
    ranges
        .iter()
        .find(|item| item.name == range_name)
        .map(|item| item.start)
}

fn build_physical_ranges(bookmarks: &[Bookmark]) -> (Vec<PageRange>, i32) {
    let chapter1_start = bookmark_page(bookmarks, "Chapter 1. ");
    let appendix_start = bookmark_page(bookmarks, "Appendix A:");

    let page_offset = chapter1_start - 13;
    if page_offset < 0 {
        die(&format!("calculated negative page offset: {page_offset}"));
    }

    let front_matter_end = chapter1_start - 1;
    if front_matter_end < 1 {
        die(&format!("invalid front matter end page: {front_matter_end}"));
    }

    let mut physical_ranges = vec![PageRange {
        name: "00-front-matter",
        start: 1,
        end: front_matter_end,
    }];

    for range in printed_ranges() {
        physical_ranges.push(PageRange {
            name: range.name,
            start: range.start + page_offset,
            end: range.end + page_offset,
        });
    }

    let appendices_start = range_start(&physical_ranges, "15-appendices")
        .unwrap_or_else(|| die("missing generated range: 15-appendices"));
    if appendices_start != appendix_start {
        die(&format!(
            "appendix start mismatch: range starts at {appendices_start}, bookmark starts at {appendix_start}"
        ));
    }

    (physical_ranges, page_offset)
}

fn validate_ranges(ranges: &[PageRange], total_pages: i32) {
    let mut previous_end = 0;
    for (index, range) in ranges.iter().enumerate() {
        let line_number = index + 1;
        if range.start > range.end {
            die(&format!(
                "line {line_number} has start > end: {} {}-{}",
                range.name, range.start, range.end
            ));
        }
        if range.start <= previous_end {
            die(&format!(
                "line {line_number} overlaps prior range: {} starts at {}, previous ended at {}",
                range.name, range.start, previous_end
            ));
        }
        if range.start != previous_end + 1 {
            die(&format!(
                "line {line_number} has gap before {}: expected {}, got {}",
                range.name,
                previous_end + 1,
                range.start
            ));
        }
        if range.end > total_pages {
            die(&format!(
                "line {line_number} exceeds source page count ({total_pages}): {} ends at {}",
                range.name, range.end
            ));
        }
        previous_end = range.end;
    }

    if previous_end < total_pages {
        warn(&format!(
            "ranges end at {previous_end} but source has {total_pages} pages (trailing pages excluded)"
        ));
    }
}

fn validate_bookmark_alignment(ranges: &[PageRange], bookmarks: &[Bookmark]) {
    for check in bookmark_checks() {
        let expected = bookmark_page(bookmarks, check.title_prefix);
        let actual = range_start(ranges, check.range_name)
            .unwrap_or_else(|| die(&format!("could not find generated range: {}", check.range_name)));
        if actual != expected {
            die(&format!(
                "bookmark mismatch for {}: range starts at {actual}, bookmark starts at {expected}",
                check.range_name
            ));
        }
    }
}

fn split_range(src_path: &Path, out_dir: &Path, range: &PageRange) {
    let pages = range.end - range.start + 1;
    println!("  {} ({}pp: {}-{})", range.name, pages, range.start, range.end);
    let output_path = out_dir.join(format!("{}.pdf", range.name));
    let page_spec = format!("{}-{}", range.start, range.end);
    run(
        "qpdf",
        &[
            src_path.to_string_lossy().as_ref(),
            "--pages",
            ".",
            page_spec.as_str(),
            "--",
            output_path.to_string_lossy().as_ref(),
        ],
    );
}

fn human_size(size_bytes: u64) -> String {
    let units = ["B", "K", "M", "G", "T"];
    let mut size = size_bytes as f64;
    let mut unit_index = 0usize;
    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    if unit_index == 0 {
        format!("{}{}", size as u64, units[unit_index])
    } else {
        format!("{size:.1}{}", units[unit_index])
    }
}

fn default_base_dir() -> PathBuf {
    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Ok(executable_path) = env::current_exe() {
        if let Some(parent) = executable_path.parent() {
            candidates.push(parent.to_path_buf());
        }
    }

    if let Ok(current_dir) = env::current_dir() {
        candidates.push(current_dir.clone());
        candidates.push(current_dir.join("rp2350-reference"));
    }

    for candidate in &candidates {
        if candidate.join("rp2350-datasheet.pdf").is_file() {
            return candidate.clone();
        }
    }

    candidates.into_iter().next().unwrap_or_else(|| PathBuf::from("."))
}

fn total_pages(src_path: &Path) -> i32 {
    let output = run_capture("qpdf", &["--show-npages", src_path.to_string_lossy().as_ref()]);
    output.trim().parse::<i32>().unwrap_or_else(|_| {
        die(&format!("invalid total page count from qpdf: {}", output.trim()))
    })
}

fn list_output_pdfs(out_dir: &Path) {
    let mut entries: Vec<(String, u64)> = Vec::new();
    let read_dir = fs::read_dir(out_dir).unwrap_or_else(|error| {
        die(&format!(
            "failed to list output directory {}: {error}",
            out_dir.to_string_lossy()
        ))
    });

    for result in read_dir {
        let entry = result.unwrap_or_else(|error| {
            die(&format!(
                "failed to read output directory entry in {}: {error}",
                out_dir.to_string_lossy()
            ))
        });
        let path = entry.path();
        if path.extension() != Some(OsStr::new("pdf")) {
            continue;
        }
        let metadata = entry.metadata().unwrap_or_else(|error| {
            die(&format!(
                "failed to stat output file {}: {error}",
                path.to_string_lossy()
            ))
        });
        entries.push((entry.file_name().to_string_lossy().to_string(), metadata.len()));
    }

    entries.sort_by_key(|item| Reverse(item.1));
    for (name, size) in entries {
        println!("{:>5} {}", human_size(size), name);
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() > 3 {
        let binary_name = Path::new(&arguments[0])
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("split-datasheet.rs");
        eprintln!("usage: {binary_name} [source.pdf] [output_dir]");
        std::process::exit(2);
    }

    let base_dir = default_base_dir();
    let mut src_path = base_dir.join("rp2350-datasheet.pdf");
    let mut out_dir = base_dir.join("datasheet");

    if arguments.len() >= 2 {
        src_path = PathBuf::from(&arguments[1]);
    }
    if arguments.len() >= 3 {
        out_dir = PathBuf::from(&arguments[2]);
    }

    require_commands(&["qpdf", "pdftk"]);
    if !src_path.is_file() {
        die(&format!(
            "source PDF not found: {}",
            src_path.to_string_lossy()
        ));
    }

    let bookmark_dump = run_capture("pdftk", &[src_path.to_string_lossy().as_ref(), "dump_data"]);
    let bookmarks = parse_bookmarks(&bookmark_dump);
    let (ranges, page_offset) = build_physical_ranges(&bookmarks);
    validate_ranges(&ranges, total_pages(&src_path));
    validate_bookmark_alignment(&ranges, &bookmarks);

    fs::create_dir_all(&out_dir).unwrap_or_else(|error| {
        die(&format!(
            "failed to create output directory {}: {error}",
            out_dir.to_string_lossy()
        ))
    });

    println!(
        "Splitting RP2350 datasheet from: {} (page offset: +{})",
        src_path.to_string_lossy(),
        page_offset
    );
    println!();

    for range in &ranges {
        split_range(&src_path, &out_dir, range);
    }

    println!();
    println!("Done! Files in: {}", out_dir.to_string_lossy());
    list_output_pdfs(&out_dir);
}
