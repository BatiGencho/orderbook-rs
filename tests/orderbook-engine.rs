use anyhow::{Context, Result};
use lobster::{BookDepth, BookLevel, FillMetadata, OrderBook, OrderEvent, OrderType, Side};
use uuid::Uuid;

fn print(ob: &OrderBook) {
    println!("Book depth {:?}", ob.depth(100));
    println!("Last trade {:?}", ob.last_trade());
    println!("Max bid {:?}", ob.max_bid());
    println!("Min ask {:?}", ob.min_ask());
}

pub mod tests {

    use super::*;

    #[test]
    fn test_orderbook() {
        // create orderbook
        let mut ob = OrderBook::new(100, 100, true);

        let oder1_id = Uuid::new_v4().as_u128();
        let order1 = OrderType::Market {
            id: oder1_id,
            qty: 2,
            side: Side::Bid,
        }; // selling
        let ev = ob.execute(order1);
        println!("\n========> Order1 {:?}", ev);

        print(&ob);

        let oder2_id = Uuid::new_v4().as_u128();
        let order2 = OrderType::Limit {
            id: oder2_id,
            qty: 3,
            price: 120,
            side: Side::Ask,
        }; // wants to buy for a given price
        let ev = ob.execute(order2);
        println!("\n========> Order2 {:?}", ev);

        print(&ob);

        let oder3_id = Uuid::new_v4().as_u128();
        let order3 = OrderType::Market {
            id: oder3_id,
            qty: 4,
            side: Side::Bid,
        }; // selling
        let ev = ob.execute(order3);
        println!("\n========> Order3 {:?}", ev);

        print(&ob);
    }
}
