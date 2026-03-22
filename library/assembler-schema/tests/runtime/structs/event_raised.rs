#[cfg(test)]
mod event_raised {
    use crate::utils::de;
    use assembler_schema::schema::prelude::*;

    #[test]
    fn instantly() {
        let e: EventRaised = de(r#"{
            "name": "on_biter_base_built",
            "order": 0,
            "description": "Raised with the provided arguments.",
            "timeframe": "instantly",
            "optional": false
        }"#);
        assert_eq!(e.name, "on_biter_base_built");
        assert!(matches!(e.timeframe, TimeFrame::Instantly));
        assert!(!e.optional);
    }

    #[test]
    fn current_tick() {
        let e: EventRaised = de(r#"{
            "name": "on_entity_damaged",
            "order": 0,
            "description": "",
            "timeframe": "current_tick",
            "optional": true
        }"#);
        assert!(matches!(e.timeframe, TimeFrame::CurrentTick));
        assert!(e.optional);
    }

    #[test]
    fn future_tick() {
        let e: EventRaised = de(r#"{
            "name": "on_entity_died",
            "order": 0,
            "description": "",
            "timeframe": "future_tick",
            "optional": false
        }"#);
        assert!(matches!(e.timeframe, TimeFrame::FutureTick));
    }
}
