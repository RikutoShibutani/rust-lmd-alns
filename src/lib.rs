use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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
