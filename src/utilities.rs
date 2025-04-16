use aliyun_sts_rust_sdk::{AssumeRoleRequest, AssumeRoleResponseCredentials, Effects, Policy, StatementBlock, StringOrArray, StsClient, Versions};
use chrono::{Local, NaiveDateTime};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use uuid::Uuid;
use std::env;
use crate::errors::{ APIResult, AppError};
use std::time::{SystemTime, UNIX_EPOCH};
use log::{error, info};

pub fn generate_unique_numbers(length: usize) -> Vec<usize> {
    if length < 4 {
        panic!("长度必须大于等于4");
    }

    // 使用系统时间作为种子
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    // 简单的线性同余生成器
    let mut rng = seed;
    let a: u64 = 1664525;
    let c: u64 = 1013904223;
    let m: u64 = 1 << 32;

    let mut numbers: Vec<usize> = Vec::with_capacity(4);
    let mut attempts = 0;
    let max_attempts = length * 4;  // 防止无限循环

    while numbers.len() < 4 && attempts < max_attempts {
        // 生成下一个随机数
        rng = (a.wrapping_mul(rng).wrapping_add(c)) % m;
        let num = (rng as usize) % length;

        // 检查是否重复
        if !numbers.contains(&num) {
            numbers.push(num);
        }
        attempts += 1;
    }

    if numbers.len() < 4 {
        panic!("无法生成足够的不重复随机数");
    }

    numbers
}


pub fn current_ts() -> i32 {
    Local::now().timestamp() as i32
}

pub fn str_to_uuid(uid: &str) -> Result<Uuid, AppError> {
    if let Ok(uuid) = Uuid::parse_str(uid) {
        Ok(uuid)
    }else{
        Err(AppError::LogicError("非法的UUID".to_string()))
    }
}

pub fn zero_uuid() -> Uuid {
    Uuid::from_bytes([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ])
}

pub fn uuid_is_zero(uuid: &Uuid) -> bool {
    uuid.as_bytes().iter().all(|&byte| byte == 0)
}

pub async fn get_sts_token(bucket: &String, file_key: &String) -> Result<AssumeRoleResponseCredentials, AppError> {
    let aid = "";
    let asec = "";
    let arn = "";

    //let req = AssumeRoleRequest::new(&arn, role_session_name, Some(policy), 3600);
    let client = StsClient::new("sts.cn-chengdu.aliyuncs.com", &aid, &asec);
    match client.sts_for_put_object(arn, bucket.as_str(), file_key.as_str(), 3600).await {
        Ok(crid)=>{
            Ok(crid)
        },
        Err(e)=>{
            error!("申请STS Token 报错");
            Err(e.into())
        }
    }
}

