use soloud::{AudioExt, LoadExt, Soloud, Wav};

const FLAG_SOUND: &[u8] = include_bytes!("../assets/flag.wav");
const UNFLAG_SOUND: &[u8] = include_bytes!("../assets/unflag.wav");

const OPEN_SOUND: &[u8] = include_bytes!("../assets/open.wav");
const OPEN_FROM_FLAGS_SOUND: &[u8] = include_bytes!("../assets/open_from_flags.wav");

const LOSE_SOUND: &[u8] = include_bytes!("../assets/lose.wav");
const WIN_SOUND: &[u8] = include_bytes!("../assets/win.wav");

pub(crate) enum Sounds {
    Flag,
    Unflag,

    Open,
    OpenFromFlags,

    Win,
    Lose,
}

pub(crate) struct SoundPlayer {
    flag: Wav,
    unflag: Wav,

    open: Wav,
    open_from_flags: Wav,

    win: Wav,
    lose: Wav,
}

impl SoundPlayer {
    pub(crate) fn new() -> Self {
        let mut flag = Wav::default();
        let mut unflag = Wav::default();

        let mut open = Wav::default();
        let mut open_from_flags = Wav::default();

        let mut win = Wav::default();
        let mut lose = Wav::default();

        flag.load_mem(FLAG_SOUND).unwrap();
        unflag.load_mem(UNFLAG_SOUND).unwrap();

        open.load_mem(OPEN_SOUND).unwrap();
        open_from_flags.load_mem(OPEN_FROM_FLAGS_SOUND).unwrap();

        win.load_mem(WIN_SOUND).unwrap();
        lose.load_mem(LOSE_SOUND).unwrap();

        Self {
            flag,
            unflag,

            open,
            open_from_flags,

            win,
            lose,
        }
    }

    pub(crate) fn play(&self, sound: Sounds) {
        static mut SOLOUD: Option<Soloud> = None;

        unsafe {
            if SOLOUD.is_none() {
                SOLOUD = Some(Soloud::default().unwrap());
            }
        }

        unsafe { SOLOUD.as_ref().unwrap() }.play(match sound {
            Sounds::Flag => &self.flag,
            Sounds::Unflag => &self.unflag,

            Sounds::Open => &self.open,
            Sounds::OpenFromFlags => &self.open_from_flags,

            Sounds::Win => &self.win,
            Sounds::Lose => &self.lose,
        });
    }
}
