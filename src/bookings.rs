#[derive(Default)]
pub struct Bookings {
    pub booking: Booking,
    pub bookings: Vec<Booking>,
    pub tbox: String,
}

#[derive(Debug, Clone, Default)]
pub struct Booking {
    pub id: usize,
    pub area: u32,
    pub weeks: u32,
    pub rooms: u32,
    pub address: String,
    pub date: String,
    pub phone: String,
    pub owner: String,
    pub alarm: bool,
    pub maintenance: bool,
}

impl Booking {
    pub fn clear(&mut self) {
        self.id = 0;
        self.area = 0;
        self.weeks = 0;
        self.rooms = 0;
        self.address = String::new();
        self.date = String::new();
        self.phone = String::new();
        self.owner = String::new();
        self.alarm = false;
        self.maintenance = false;
    }

    pub fn get_id(&self) -> String {
        if self.id == 0 {
            return String::new();
        }
        format!("{}", self.id)
    }

    pub fn get_area(&self) -> String {
        if self.area == 0 {
            return String::new();
        }
        format!("{}", self.area)
    }

    pub fn get_weeks(&self) -> String {
        if self.weeks == 0 {
            return String::new();
        }
        format!("{}", self.weeks)
    }

    pub fn get_rooms(&self) -> String {
        if self.rooms == 0 {
            return String::new();
        }
        format!("{}", self.rooms)
    }

    pub fn get_booking(&self) -> String {
        format!(
            "id: {}, area: {}, weeks: {}, rooms: {}, address: {}, date: {}, phone: {}, owner: {}, alarm: {}, pool: {}",
            self.id,
            self.area,
            self.weeks,
            self.rooms,
            self.address,
            self.date,
            self.phone,
            self.owner,
            self.alarm,
            self.maintenance,
        )
    }
}
