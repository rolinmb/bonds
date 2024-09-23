use std::f64;

const FACEVAL: f64 = 1000.0;
const COUPONRATE: f64 = 0.05;
const MARKETRATE: f64 = 0.04;
const YEARSTOMAT: i32 = 10;

fn get_bond_price(face_val: f64, coupon_rate: f64, market_rate: f64, years_to_maturity: i32, compound: bool) -> f64 {
    let annual_coupon = face_val * coupon_rate;
    let mut price = 0.0;
    if compound {
        for t in 1..=years_to_maturity {
            price += annual_coupon * (1.0 + market_rate).powi(years_to_maturity - t) / (1.0 + market_rate).powi(t);
        }
    } else {
        for t in 1..=years_to_maturity {
            price += annual_coupon / (1.0 + market_rate).powi(t);
        }
    }
    price += face_val / (1.0 + market_rate).powi(years_to_maturity);
    price
}

fn bond_maturity_value(face_val: f64, coupon_rate: f64, years_to_maturity: i32, compound: bool) -> f64 {
    let annual_coupon = face_val * coupon_rate;
    if compound {
        let mut total_compounding = face_val;
        for t in 1..=years_to_maturity {
            total_compounding += annual_coupon * (1.0 + coupon_rate).powi(years_to_maturity - t);
        }
        total_compounding
    } else {
        face_val + (annual_coupon * years_to_maturity as f64)
    }
}

fn main() {
    let bond_price_nc = get_bond_price(FACEVAL, COUPONRATE, MARKETRATE, YEARSTOMAT, false);
    let mat_val_nc = bond_maturity_value(FACEVAL, COUPONRATE, YEARSTOMAT, false);
    let bond_price_c = get_bond_price(FACEVAL, COUPONRATE, MARKETRATE, YEARSTOMAT, true);
    let mat_val_c = bond_maturity_value(FACEVAL, COUPONRATE, YEARSTOMAT, true);
    println!("\nmain() :: NON-COMPOUNDING ~ Face Value: ${:.2} Coupon Rate: {} (%) Market Rate: {} (%) Years to Maturity: {}\n\t=> Bond Price: ${:.2} Value at Maturity: ${:.}", FACEVAL, COUPONRATE, MARKETRATE, YEARSTOMAT, bond_price_nc, mat_val_nc);
    println!("\nmain() :: COMPOUNDING ~ Face Value: ${:.2} Coupon Rate: {} (%) Market Rate: {} (%) Years to Maturity: {}\n\t=> Bond Price: ${:.2} Value at Maturity: ${:.2}", FACEVAL, COUPONRATE, MARKETRATE, YEARSTOMAT, bond_price_c, mat_val_c);
}
