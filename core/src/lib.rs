pub mod tic_tac_toe;
pub mod game_message;
pub mod user;

pub fn add(left: usize, right: usize) -> usize {
    println!("add fn called in core");
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
