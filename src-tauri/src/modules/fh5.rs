use super::GameModule;

pub struct FH5Module;

impl GameModule for FH5Module {
    fn target_process_name(&self) -> &'static str {
        "ForzaHorizon5.exe"
    }

    fn discord_client_id(&self) -> &'static str {
        "1501532618989113434"
    }

    fn uwp_package_name(&self) -> &'static str {
        "Microsoft.624F8B84B80_8wekyb3d8bbwe"
    }

    fn game_name(&self) -> &'static str {
        "Forza Horizon 5"
    }

    fn format_class(&self, class_id: i32) -> String {
        match class_id {
            0 => "E".into(),
            1 => "D".into(),
            2 => "C".into(),
            3 => "B".into(),
            4 => "A".into(),
            5 => "S1".into(),
            6 => "S2".into(),
            7 => "X".into(),
            _ => "Unknown".into(),
        }
    }
}
