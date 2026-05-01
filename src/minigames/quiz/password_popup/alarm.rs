use bevy::{prelude::*};

// const ALARM_FILE_NAME: &'static str = "quiz_game/sounds/attenzione-pickpocket.ogg";
const ALARM_FILE_NAME: &'static str = "quiz_game/sounds/mixkit-classic-alarm-995.ogg";

pub fn start_alarm(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("spawned alarm!");
    commands.spawn((
        AudioPlayer::new(asset_server.load(ALARM_FILE_NAME)),
        PlaybackSettings::DESPAWN,
    ));
}