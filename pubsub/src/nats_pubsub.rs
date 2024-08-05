use nats::Connection;

pub struct PubSub {
    nats: Connection,
}

impl PubSub {
    pub fn new(url: &str) -> Self {
        let nats = nats::connect(url).unwrap();
        PubSub { nats }
    }

    pub fn publish(&self, subject: &str, message: &str) {
        self.nats.publish(subject, message).unwrap();
    }

    pub fn subscribe(&self, subject: &str) -> nats::Subscription {
        self.nats.subscribe(subject).unwrap()
    }
}
