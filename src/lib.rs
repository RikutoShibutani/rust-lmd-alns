use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub struct customers
// 顧客情報
{
    id: usize,
    x: usize,
    y: usize,
    ready_time: usize,
    due_time: usize,
    service: usize,
}

pub struct Cluster
// クラスタ情報
{
    id: usize,
    customers: Vec<usize>,
    x: usize,
    y: usize,
    demand: usize,
    ready_time: usize,
    due_time: usize,
    service: usize,
}

pub struct Instance
// インスタンス情報
{
    clusters: Vec<usize>,
    parkingnum: usize,
    maxvehiclenum: usize,
    maxdeliverynum: usize,
    maxvehiclecapacity: usize,
    vehiclefixedcost: f64,
    deliveryfixedcost: f64,
    vehicleunitcost: f64,
    deliveryunitcost: f64,
    filename: String,
}

pub struct Tour
// 経路情報
{
    visitedClusterID: usize,
    visitedCustomerID: Vec<usize>,
}

pub struct Delivery
// 配達情報
{
    deliveryID: usize,
    deliveryRoutecost: f64,
    time: f64,
}

pub struct Vehicle
// 車両情報
{
    vehicleID: usize,
    numdeliveryman: usize,
    vehicleRoutecost: f64,
    capacity: usize,
    time: f64,
    deliverymen: Vec<Delivery>,
    tour: Vec<Tour>,
}

pub fn read_instance(filename: &str) -> Instance {
    let mut instance: Instance = Instance {};
    let mut file: File = File::open(filename).unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut line: String = String::new();
    reader.read_line(&mut line).unwrap();
    let mut tokens: Vec<&str> = line.split_whitespace().collect();
    instance.parkingnum = tokens[0].parse().unwrap();
    instance.maxvehiclenum = tokens[1].parse().unwrap();
    instance.maxdeliverynum = tokens[2].parse().unwrap();
    instance.maxvehiclecapacity = tokens[3].parse().unwrap();
    instance.vehiclefixedcost = tokens[4].parse().unwrap();
    instance.deliveryfixedcost = tokens[5].parse().unwrap();
    instance.vehicleunitcost = tokens[6].parse().unwrap();
    instance.deliveryunitcost = tokens[7].parse().unwrap();
    instance.filename = filename.to_string();
}
