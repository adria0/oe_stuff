#[macro_use]
use serde_json::json;
use serde::{Deserialize};
use anyhow::Result;
use tokio::time;
use psutil::{
    disk::{self, DiskIoCountersCollector},
    process::{ProcessCollector},
};
use std::time::{SystemTime, UNIX_EPOCH};

async fn rpc_call<T>(host: &str, method: &str) -> Result<T>
where T : for<'de> serde::Deserialize<'de> {
    let client = reqwest::Client::new();
    let mut map = std::collections::HashMap::new();
    map.insert("jsonrpc", json!("2.0"));
    map.insert("method", json!(method));
    map.insert("params", json!(serde_json::value::Value::Array(Vec::new())));
    map.insert("id",json!("1"));
    let res = client.post(host)
        .json(&map)
        .send()
        .await?
        .json::<T>()
        .await?;
    Ok(res)
}

#[derive(Deserialize)]
struct PeerCount {
    result : String,
}
impl PeerCount {
    pub async fn peer_count(host: &str) -> Result<u64> {
        let res: PeerCount = rpc_call(host,"net_peerCount").await?;
        Ok(u64::from_str_radix(&res.result[2..], 16)?)
    }
}

#[tokio::main]
async fn main() -> Result<()> {

    let pid = u32::from_str_radix(
        &std::env::args().nth(1).expect("pid requiered"),
        10,
    ).expect("pid is not a number");

    let rpc_url = "http://localhost:8545";

    let mut disk_io_counters_collector = DiskIoCountersCollector::default();
    let mut last_disk_io = disk_io_counters_collector.disk_io_counters()?;

    loop {
        let disk_usage = disk::disk_usage("/")?;

        let disk_io = disk_io_counters_collector.disk_io_counters()?;
        let disk_io_diff = disk_io.clone() - last_disk_io;
        last_disk_io = disk_io;


        time::delay_for(std::time::Duration::from_secs(5)).await;

        let mut process_collector = ProcessCollector::new()?;
        let (cpu,rss, vms, open_files) = if let Some(process) =  process_collector.processes.get_mut(&pid) {
            let process_mem_info = process.memory_info()?;
            (process.cpu_percent()? as i64,process_mem_info.rss() as i64,process_mem_info.vms() as i64, process.open_files()?.len() as i64)
        } else {
            (-1 ,-1 ,-1 ,-1)
        };

        let peer_count = PeerCount::peer_count(rpc_url)
            .await
            .map_or(-1,|v| v as i64);

        println!("{:?} cpu:{:?} rss:{:?} vms:{:?} files:{:?} read:{:?} write:{:?} disk_used:{:?} peers:{:?}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            cpu, rss, vms, open_files,
            disk_io_diff.read_count(), disk_io_diff.write_count(),disk_usage.used(),
            peer_count
        );
    }

}