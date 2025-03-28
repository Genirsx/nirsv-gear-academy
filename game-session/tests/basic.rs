#[cfg(test)]
mod tests {
    use game_session_io::*;
    use gstd::{prelude::*, ActorId};
    use gtest::{Program, System};

    const WORDLE_ID: u64 = 100;
    //const GAME_SESSION_ID: u64 = 200;
    const USER1: u64 = 10;

    #[test]
    fn test_start_game() {
        let sys = System::new();
        sys.init_logger();

        let wordle = Program::from_file(
            &sys,
            "../target/wasm32-unknown-unknown/debug/wordle.opt.wasm",
        );
        let game_session = Program::from_file(
            &sys,
            "../target/wasm32-unknown-unknown/debug/game_session.opt.wasm",
        );

        let user_id: ActorId = USER1.into();
        let wordle_id: ActorId = WORDLE_ID.into();

        sys.mint_to(user_id, 100000000000000000000);
        wordle.send(user_id, wordle_id);
        sys.run_next_block();
        game_session.send(user_id, wordle_id);
        sys.run_next_block();

        //let game_session = game_session.get_program(GAME_SESSION_ID).expect("Game session not found");
        game_session.send(USER1, GameSessionAction::StartGame);
        sys.run_next_block();
        let state: GameSessionState = game_session.read_state(()).expect("Failed to read state");
        assert!(state
            .game_sessions
            .iter()
            .any(|(user, _)| *user == USER1.into()));
        println!("as: {:?}", state);

        let session_info = &state
            .game_sessions
            .iter()
            .find(|(user, _)| *user == USER1.into())
            .unwrap()
            .1;
        assert!(matches!(
            session_info.session_status,
            SessionStatus::WaitWordleStartReply
        ));
    }

    #[test]
    fn test_check_word_correct_check_win() {
        let sys = System::new();
        sys.init_logger();

        let wordle = Program::from_file(
            &sys,
            "../target/wasm32-unknown-unknown/debug/wordle.opt.wasm",
        );
        let game_session = Program::from_file(
            &sys,
            "../target/wasm32-unknown-unknown/debug/game_session.opt.wasm",
        );

        let user_id: ActorId = USER1.into();
        let wordle_id: ActorId = WORDLE_ID.into();
        sys.mint_to(user_id, 100000000000000000000);
        wordle.send(user_id, wordle_id);
        sys.run_next_block();
        game_session.send(user_id, wordle_id);
        sys.run_next_block();
        //let game_session = sys.get_program(GAME_SESSION_ID).expect("Game session not found");

        game_session.send(USER1, GameSessionAction::StartGame);
        sys.run_next_block();
        let state: GameSessionState = game_session.read_state(()).expect("Failed to read state");
        println!("111: {:?}", state);

        game_session.send(
            USER1,
            GameSessionAction::CheckWord {
                word: "hello".to_string(),
            },
        );
        sys.run_next_block();
        game_session.send(
            USER1,
            GameSessionAction::CheckWord {
                word: "hello".to_string(),
            },
        );
        sys.run_next_block();
        let state: GameSessionState = game_session.read_state(()).expect("Failed to read state");
        let session_info = &state
            .game_sessions
            .iter()
            .find(|(user, _)| *user == USER1.into())
            .unwrap()
            .1;
        println!("as: {:?}", state);

        assert_eq!(session_info.tries, 0);

        game_session.send(
            USER1,
            GameSessionAction::CheckWord {
                word: "house".to_string(),
            },
        );
        sys.run_next_block();
        let state: GameSessionState = game_session.read_state(()).expect("Failed to read state");
        let session_info = &state
            .game_sessions
            .iter()
            .find(|(user, _)| *user == USER1.into())
            .unwrap()
            .1;
        assert_eq!(session_info.tries, 0);
        assert!(matches!(
            session_info.session_status,
            SessionStatus::WaitWordleStartReply
        ));
    }

    #[test]
    fn test_game_over() {
        let sys = System::new();
        sys.init_logger();

        let wordle = Program::from_file(
            &sys,
            "../target/wasm32-unknown-unknown/debug/wordle.opt.wasm",
        );
        let game_session = Program::from_file(
            &sys,
            "../target/wasm32-unknown-unknown/debug/game_session.opt.wasm",
        );

        let user_id: ActorId = USER1.into();
        let wordle_id: ActorId = WORDLE_ID.into();
        sys.mint_to(user_id, 100000000000000000000);
        wordle.send(user_id, wordle_id);
        sys.run_next_block();
        game_session.send(user_id, wordle_id);
        sys.run_next_block();
        //let game_session = sys.get_program(GAME_SESSION_ID).expect("Game session not found");

        game_session.send(USER1, GameSessionAction::StartGame);
        sys.run_next_block();
        game_session.send(
            USER1,
            GameSessionAction::CheckWord {
                word: "hello".to_string(),
            },
        );
        sys.run_next_block();
        game_session.send(
            USER1,
            GameSessionAction::CheckWord {
                word: "wrong".to_string(),
            },
        );
        sys.run_next_block();
        game_session.send(
            USER1,
            GameSessionAction::CheckWord {
                word: "wrong".to_string(),
            },
        );
        sys.run_next_block();
        game_session.send(
            USER1,
            GameSessionAction::CheckWord {
                word: "wrong".to_string(),
            },
        );
        sys.run_next_block();
        game_session.send(
            USER1,
            GameSessionAction::CheckWord {
                word: "wrong".to_string(),
            },
        );
        sys.run_next_block();
        let state: GameSessionState = game_session.read_state(()).expect("Failed to read state");
        println!("111: {:?}", state);
        let session_info = &state
            .game_sessions
            .iter()
            .find(|(user, _)| *user == USER1.into())
            .unwrap()
            .1;
        assert!(matches!(
            session_info.session_status,
            SessionStatus::WaitWordleStartReply
        ));
    }

    #[test]
    fn test_time() {
        let sys = System::new();
        sys.init_logger();

        let wordle = Program::from_file(
            &sys,
            "../target/wasm32-unknown-unknown/debug/wordle.opt.wasm",
        );
        let game_session = Program::from_file(
            &sys,
            "../target/wasm32-unknown-unknown/debug/game_session.opt.wasm",
        );

        let user_id: ActorId = USER1.into();
        let wordle_id: ActorId = WORDLE_ID.into();
        sys.mint_to(user_id, 100000000000000000000);
        wordle.send(user_id, wordle_id);
        sys.run_next_block();
        game_session.send(user_id, wordle_id);
        sys.run_next_block();
        //let game_session = sys.get_program(GAME_SESSION_ID).expect("Game session not found");

        game_session.send(USER1, GameSessionAction::StartGame);
        sys.run_next_block();
        sys.run_to_block(200);

        let state: GameSessionState = game_session.read_state(()).expect("Failed to read state");
        let session_info = &state
            .game_sessions
            .iter()
            .find(|(user, _)| *user == USER1.into())
            .unwrap()
            .1;
        println!("as: {:?}", state);

        assert!(matches!(
            session_info.session_status,
            SessionStatus::WaitWordleStartReply
        ));
    }
}
