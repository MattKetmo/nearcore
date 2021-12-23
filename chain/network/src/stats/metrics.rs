use once_cell::sync::Lazy;
use strum::VariantNames;

pub static PEER_CONNECTIONS_TOTAL: Lazy<near_metrics::IntGauge> = Lazy::new(|| {
    near_metrics::try_create_int_gauge("near_peer_connections_total", "Number of connected peers")
        .unwrap()
});
pub static PEER_DATA_RECEIVED_BYTES: Lazy<near_metrics::IntCounter> = Lazy::new(|| {
    near_metrics::try_create_int_counter(
        "near_peer_data_received_bytes",
        "Total data received from peers",
    )
    .unwrap()
});
pub static PEER_MESSAGE_RECEIVED_TOTAL: Lazy<near_metrics::IntCounter> = Lazy::new(|| {
    near_metrics::try_create_int_counter(
        "near_peer_message_received_total",
        "Number of messages received from peers",
    )
    .unwrap()
});
pub static PEER_CLIENT_MESSAGE_RECEIVED_TOTAL: Lazy<near_metrics::IntCounter> = Lazy::new(|| {
    near_metrics::try_create_int_counter(
        "near_peer_client_message_received_total",
        "Number of messages for client received from peers",
    )
    .unwrap()
});
pub static PEER_BLOCK_RECEIVED_TOTAL: Lazy<near_metrics::IntCounter> = Lazy::new(|| {
    near_metrics::try_create_int_counter(
        "near_peer_block_received_total",
        "Number of blocks received by peers",
    )
    .unwrap()
});
pub static PEER_TRANSACTION_RECEIVED_TOTAL: Lazy<near_metrics::IntCounter> = Lazy::new(|| {
    near_metrics::try_create_int_counter(
        "near_peer_transaction_received_total",
        "Number of transactions received by peers",
    )
    .unwrap()
});

// Routing table metrics
pub static ROUTING_TABLE_RECALCULATIONS: Lazy<near_metrics::IntCounter> = Lazy::new(|| {
    near_metrics::try_create_int_counter(
        "near_routing_table_recalculations_total",
        "Number of times routing table have been recalculated from scratch",
    )
    .unwrap()
});
pub static ROUTING_TABLE_RECALCULATION_HISTOGRAM: Lazy<near_metrics::Histogram> = Lazy::new(|| {
    near_metrics::try_create_histogram(
        "near_routing_table_recalculation_seconds",
        "Time spent recalculating routing table",
    )
    .unwrap()
});
pub static EDGE_UPDATES: Lazy<near_metrics::IntCounter> = Lazy::new(|| {
    near_metrics::try_create_int_counter("near_edge_updates", "Unique edge updates").unwrap()
});
pub static EDGE_ACTIVE: Lazy<near_metrics::IntGauge> = Lazy::new(|| {
    near_metrics::try_create_int_gauge("near_edge_active", "Total edges active between peers")
        .unwrap()
});
pub static PEER_REACHABLE: Lazy<near_metrics::IntGauge> = Lazy::new(|| {
    near_metrics::try_create_int_gauge(
        "near_peer_reachable",
        "Total peers such that there is a path potentially through other peers",
    )
    .unwrap()
});
pub static DROP_MESSAGE_UNKNOWN_ACCOUNT: Lazy<near_metrics::IntCounter> = Lazy::new(|| {
    near_metrics::try_create_int_counter(
        "near_drop_message_unknown_account",
        "Total messages dropped because target account is not known",
    )
    .unwrap()
});
pub static RECEIVED_INFO_ABOUT_ITSELF: Lazy<near_metrics::IntCounter> = Lazy::new(|| {
    near_metrics::try_create_int_counter(
        "received_info_about_itself",
        "Number of times a peer tried to connect to itself",
    )
    .unwrap()
});
pub static DROPPED_MESSAGES_COUNT: Lazy<near_metrics::IntCounter> = Lazy::new(|| {
    near_metrics::try_create_int_counter(
        "near_dropped_messages_count",
        "Total count of messages which were dropped, because write buffer was full",
    )
    .unwrap()
});

#[derive(Clone)]
pub struct NetworkMetrics {
    pub peer_messages: std::collections::HashMap<String, Option<near_metrics::IntCounter>>,
}

impl NetworkMetrics {
    pub fn new() -> Self {
        let metrics = [
            NetworkMetrics::peer_message_total_rx,
            NetworkMetrics::peer_message_bytes_rx,
            NetworkMetrics::peer_message_dropped,
        ];
        Self {
            peer_messages: (crate::types::PeerMessage::VARIANTS.iter().filter(|n| n != &&"Routed"))
                .chain(near_network_primitives::types::RoutedMessageBody::VARIANTS.iter())
                .flat_map(|x| metrics.map(|method| method(x)))
                .map(|cn| (cn.clone(), near_metrics::try_create_int_counter(&cn, &cn).ok()))
                .collect(),
        }
    }

    pub fn peer_message_total_rx(message_name: &str) -> String {
        format!("near_{}_total", message_name.to_lowercase())
    }

    pub fn peer_message_bytes_rx(message_name: &str) -> String {
        format!("near_{}_bytes", message_name.to_lowercase())
    }

    pub fn peer_message_dropped(message_name: &str) -> String {
        format!("near_{}_dropped", message_name.to_lowercase())
    }

    pub fn inc(&self, message_name: &str) {
        self.peer_messages.get(message_name).iter().for_each(|c| c.as_ref().unwrap().inc());
    }

    pub fn inc_by(&self, message_name: &str, value: u64) {
        if let Some(counter) = self.peer_messages.get(message_name) {
            near_metrics::inc_counter_by_opt(counter.as_ref(), value);
        }
    }
}
