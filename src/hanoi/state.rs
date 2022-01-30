use std::{io, fmt};
use super::{Move, Rod, Disk};

pub struct State {
    rods: [Vec<Disk>; Rod::count()],
}

impl State {
    pub fn new(count: Disk) -> io::Result<Self> {
        const MAX_COUNT: Disk = Disk::BITS as Disk;
        const MIN_COUNT: Disk = 1;
        if MIN_COUNT > count || count > MAX_COUNT {
            return Err(io::Error::from(io::ErrorKind::InvalidInput));//, "You must use count of disks in range [{}; {}]", MIN_COUNT, MAX_COUNT);
        }

        let mut rods = [Vec::with_capacity(count as usize), Vec::with_capacity(count as usize), Vec::with_capacity(count as usize)];
        for i in 1..=count {
            rods[0].push(i);
        }

        Ok(Self { rods })
    }

    pub fn top_on(&self, rod: &Rod) -> &Disk {
        self.rods[usize::from(rod.clone())].last().unwrap_or(&0)
    }

    pub fn move_disk(&mut self, mv: &Move) -> io::Result<()> {
        let from: usize = mv.from.clone().into();
        let to  : usize = mv.to  .clone().into();

        // Проверяем, что стержень не пустой
        let movable = self.rods[from]
            .last()
            .ok_or_else(|| io::Error::from(io::ErrorKind::Other))?;

        // Проверяем, что на целевом стержне нет диска с большим номером
        match self.rods[to].last() {
            Some(top) if top > movable => Err(io::Error::from(io::ErrorKind::Other)),
            _ => Ok(()),
        }?;

        // Осуществляем реальный перенос
        let movable = self.rods[from].pop().unwrap();
        self.rods[to].push(movable);

        Ok(())
    }
}

impl Default for State {
    fn default() -> Self {
        State::new(3).unwrap()
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, rod) in self.rods.iter().enumerate() {
            write!(f, "Rod{}:", idx + 1)?;
            for disk in rod {
                write!(f, " {}", disk)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
