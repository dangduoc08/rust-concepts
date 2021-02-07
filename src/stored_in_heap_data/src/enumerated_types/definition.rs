#[derive(Debug)]
pub enum SyncStatus {
    Synced,
    Unsynced,
}

#[derive(Debug)]
pub enum ProductVariation {
    SyncPrice(SyncStatus),
    SyncStock(SyncStatus),
}

pub const ANY: Option<usize> = Some(20);

/*
The Option<T> enum is so useful that it’s even included in the prelude;
you don’t need to bring it into scope explicitly.
In addition, so are its variants: you can use Some and None directly
without the Option:: prefix.
The Option<T> enum is still just a regular enum,
and Some(T) and None are still variants of type Option<T>.

When we have a Some value,
we know that a value is present and the value is held within the Some.
When we have a None value, in some sense,
it means the same thing as null: we don’t have a valid value.
So why is having Option<T> any better than having null?
*/
