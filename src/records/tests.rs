#[cfg(test)]
mod records_tests {
    use crate::records::model::RunRecord;
    use crate::records::{personal_best, recent};

    fn make(lang: &str, len: &str, wpm: f32, completed: bool) -> RunRecord {
        RunRecord::new(lang, len, wpm, 95.0, 30.0, 1, completed)
    }

    #[test]
    fn test_new_record_has_timestamp() {
        let r = make("rust", "50", 85.0, true);
        assert!(!r.timestamp.is_empty());
        assert!(r.timestamp.contains('T'));
    }

    #[test]
    fn test_recent_returns_last_n() {
        let records = vec![
            make("rust", "25", 50.0, true),
            make("go", "50", 60.0, true),
            make("python", "100", 70.0, true),
        ];
        let last2 = recent::recent(&records, 2);
        assert_eq!(last2.len(), 2);
        assert_eq!(last2[0].language, "python");
        assert_eq!(last2[1].language, "go");
    }

    #[test]
    fn test_recent_more_than_available() {
        let records = vec![make("rust", "25", 50.0, true)];
        assert_eq!(recent::recent(&records, 5).len(), 1);
    }

    #[test]
    fn test_personal_best_highest_wpm() {
        let records = vec![
            make("rust", "50", 60.0, true),
            make("rust", "50", 90.0, true),
            make("rust", "50", 75.0, true),
        ];
        let pb = personal_best::personal_best(&records, "rust", "50").unwrap();
        assert!((pb.wpm - 90.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_incomplete_not_pb() {
        let records = vec![
            make("rust", "50", 60.0, true),
            make("rust", "50", 200.0, false),
        ];
        let pb = personal_best::personal_best(&records, "rust", "50").unwrap();
        assert!((pb.wpm - 60.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_personal_best_none_when_empty() {
        let records: Vec<RunRecord> = Vec::new();
        assert!(personal_best::personal_best(&records, "rust", "50").is_none());
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        use std::fs;
        let dir = std::env::temp_dir().join("10fp_test_storage");
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("roundtrip.json");

        let records = vec![make("rust", "25", 90.0, true)];
        let json = serde_json::to_string_pretty(&records).unwrap();
        fs::write(&path, &json).unwrap();

        let loaded: Vec<RunRecord> = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].language, "rust");

        let _ = fs::remove_dir_all(&dir);
    }
}
