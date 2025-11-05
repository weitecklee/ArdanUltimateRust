pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub async fn double(n: u32) -> u32 {
    2 * n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn the_hard_way() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        assert_eq!(rt.block_on(double(2)), 4);
    }

    // #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[tokio::test]
    async fn the_easy_way() {
        assert_eq!(double(2).await, 4);
    }
}
