mod crud;
mod db;

pub use db::Connection;
pub use crud::Base;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
