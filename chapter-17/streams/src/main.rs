use trpl::StreamExt;

fn main() {
    // Streams and itereators are very similar, but with the difference
    // that streams are asynchronous and iterators are synchronous.
    // We can create an stream from any iterator.
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}
