-- Your SQL goes here

CREATE TABLE if not exists orders (
  id UUID,
  orderbook_id UUID,
  created_at TIMESTAMP NOT NULL,
  ticker VARCHAR NOT NULL,
  order_type VARCHAR NOT NULL,
  order_side VARCHAR NOT NULL,
  order_status VARCHAR NOT NULL,
  order_value VARCHAR NOT NULL,
  order_quantity VARCHAR NOT NULL,
  logs JSON NOT NULL,
  PRIMARY KEY (id)
)