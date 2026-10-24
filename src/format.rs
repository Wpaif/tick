pub fn format_progress(progress: f64) -> String {
    format!("{:.0}%", progress * 100.0)
}

pub fn format_speed(speed: u64) -> String {
    if speed < 1024 {
        return format!("{} B/s", speed);
    }
    if speed < 1024 * 1024 {
        return format!("{:.1} KiB/s", speed as f64 / 1024.0);
    }
    format!("{:.1} MiB/s", speed as f64 / (1024.0 * 1024.0))
}

pub fn format_eta(eta: i64) -> String {
    if eta >= 8640000 {
        return "∞".to_string();
    }
    if eta < 60 {
        return format!("{}s", eta);
    }
    if eta < 3600 {
        return format!("{}m", eta / 60);
    }
    if eta < 3600 * 24 {
        return format!("{}h", eta / 3600);
    }
    format!("{}d", eta / (3600 * 24))
}

pub fn format_state(state: &str) -> &str {
    match state {
        "downloading" | "forcedDL" => "Downloading",
        "stoppedUP" | "stoppedDL" => "Stopped",
        "uploading" | "forcedUP" => "Seeding",
        "stalledDL" | "stalledUP" => "Stalled",
        "queuedDL" | "queuedUP" => "Queued",
        "checkingDL" | "checkingUP" | "checkingResumeData" => "Checking",
        "metaDL" => "Fetching metadata",
        "missingFiles" => "Missing files",
        "error" => "Error",
        _ => state,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_progress_as_percent() {
        assert_eq!(format_progress(0.42), "42%");
    }

    #[test]
    fn rounds_progress_to_whole_percent() {
        assert_eq!(format_progress(0.29), "29%");
    }

    #[test]
    fn formats_speed_below_1024_as_bytes() {
        assert_eq!(format_speed(500), "500 B/s");
    }

    #[test]
    fn formats_speed_in_kib() {
        assert_eq!(format_speed(2048), "2.0 KiB/s");
    }

    #[test]
    fn formats_speed_in_mib() {
        assert_eq!(format_speed(1048576), "1.0 MiB/s");
    }

    #[test]
    fn formats_unknown_eta_as_infinity() {
        assert_eq!(format_eta(8640000), "∞");
    }

    #[test]
    fn formats_eta_as_seconds() {
        assert_eq!(format_eta(45), "45s");
    }

    #[test]
    fn formats_eta_as_minutes() {
        assert_eq!(format_eta(360), "6m");
    }

    #[test]
    fn formats_eta_as_hours() {
        assert_eq!(format_eta(7200), "2h");
    }

    #[test]
    fn formats_eta_as_days() {
        assert_eq!(format_eta(86400), "1d");
    }

    #[test]
    fn formats_downloading_states() {
        assert_eq!(format_state("downloading"), "Downloading");
        assert_eq!(format_state("forcedDL"), "Downloading");
    }

    #[test]
    fn formats_stopped_states() {
        assert_eq!(format_state("stoppedUP"), "Stopped");
        assert_eq!(format_state("stoppedDL"), "Stopped");
    }

    #[test]
    fn formats_seeding_states() {
        assert_eq!(format_state("uploading"), "Seeding");
        assert_eq!(format_state("forcedUP"), "Seeding");
    }

    #[test]
    fn formats_stalled_states() {
        assert_eq!(format_state("stalledDL"), "Stalled");
        assert_eq!(format_state("stalledUP"), "Stalled");
    }

    #[test]
    fn formats_queued_states() {
        assert_eq!(format_state("queuedDL"), "Queued");
        assert_eq!(format_state("queuedUP"), "Queued");
    }

    #[test]
    fn formats_checking_states() {
        assert_eq!(format_state("checkingDL"), "Checking");
        assert_eq!(format_state("checkingUP"), "Checking");
        assert_eq!(format_state("checkingResumeData"), "Checking");
    }

    #[test]
    fn formats_fetching_metadata_states() {
        assert_eq!(format_state("metaDL"), "Fetching metadata");
    }

    #[test]
    fn formats_missing_files_states() {
        assert_eq!(format_state("missingFiles"), "Missing files");
    }

    #[test]
    fn formats_error_states() {
        assert_eq!(format_state("error"), "Error");
    }
}
