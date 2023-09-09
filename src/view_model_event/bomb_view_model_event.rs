use bevy::prelude::Event;

#[derive(Event)]
pub struct BombViewModelEvent {
    /**
     * そもそもBombを表示するかどうか
     */
    show: bool,

    /**
     * 残り時間がどれぐらい残っているか
     * 0-100
     */ 
    remain_time_percentage: f32,

    /**
     * 回答されている個数
     * 0-4
     */ 
    answered_count: u8
}

impl BombViewModelEvent {
    pub fn create(
        show: bool,
        remain_time_percentage: f32,
        answered_count: u8
    ) -> Option<Self> {
        if !Self::validation(
            show,
            remain_time_percentage,
            answered_count
        ) {
            None
        } else {
            Some(Self::new(
                show,
                remain_time_percentage,
                answered_count
            ))
        }
    }

    fn validation(
        show: bool,
        remain_time_percentage: f32,
        answered_count: u8
    ) -> bool {
        if show {
            if !(0.0..=100.0).contains(&remain_time_percentage) {
                return false;
            }
            if answered_count > 4 {
                return false;
            }
        }
        true
    }

    fn new(
        show: bool,
        remain_time_percentage: f32,
        answered_count: u8
    ) -> Self {
        Self {
            show,
            remain_time_percentage,
            answered_count
        }
    }
}
