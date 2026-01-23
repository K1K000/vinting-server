use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
};

pub struct AllRouteFairing;

#[async_trait]
impl Fairing for AllRouteFairing {
    fn info(&self) -> Info {
        Info {
            name: "All route fairing",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let r = r;
        // .attach(SomeOtherFairing);

        Ok(r)
    }
}
