use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;

pub struct Images<'lt> {
    background: Texture<'lt>,
    border: Texture<'lt>,
    cadence_1: Texture<'lt>,
    cadence_2: Texture<'lt>,
    cadence_3: Texture<'lt>,
    cadence_4: Texture<'lt>,
    diamond: Texture<'lt>,
    diamond_icon: Texture<'lt>,
    exit: Texture<'lt>,
    exit_locked: Texture<'lt>,
    floor_green: Texture<'lt>,
    floor_purple: Texture<'lt>,
    shovel: Texture<'lt>,
    stone: Texture<'lt>,
    wall: Texture<'lt>,
}

impl<'lt> Images<'lt> {
    pub fn load<T>(tc: &'lt TextureCreator<T>) -> Self {
        let mut background_data = include_raw_image!("res/background.png");
        let mut border_data = include_raw_image!("res/border.png");
        let mut cadence_1_data = include_raw_image!("res/cadence_1.png");
        let mut cadence_2_data = include_raw_image!("res/cadence_2.png");
        let mut cadence_3_data = include_raw_image!("res/cadence_3.png");
        let mut cadence_4_data = include_raw_image!("res/cadence_4.png");
        let mut diamond_data = include_raw_image!("res/diamond.png");
        let mut diamond_icon_data = include_raw_image!("res/diamond_icon.png");
        let mut exit_data = include_raw_image!("res/exit.png");
        let mut exit_locked_data = include_raw_image!("res/exit_locked.png");
        let mut floor_green_data = include_raw_image!("res/floor_green.png");
        let mut floor_purple_data = include_raw_image!("res/floor_purple.png");
        let mut shovel_data = include_raw_image!("res/shovel.png");
        let mut stone_data = include_raw_image!("res/stone.png");
        let mut wall_data = include_raw_image!("res/wall.png");

        let background_surface = Surface::from_data(
            &mut background_data,
            480,
            270,
            1920,
            PixelFormatEnum::ABGR8888,
        )
        .unwrap();
        let border_surface =
            Surface::from_data(&mut border_data, 24, 48, 96, PixelFormatEnum::ABGR8888).unwrap();
        let cadence_1_surface =
            Surface::from_data(&mut cadence_1_data, 24, 48, 96, PixelFormatEnum::ABGR8888).unwrap();
        let cadence_2_surface =
            Surface::from_data(&mut cadence_2_data, 24, 48, 96, PixelFormatEnum::ABGR8888).unwrap();
        let cadence_3_surface =
            Surface::from_data(&mut cadence_3_data, 24, 48, 96, PixelFormatEnum::ABGR8888).unwrap();
        let cadence_4_surface =
            Surface::from_data(&mut cadence_4_data, 24, 48, 96, PixelFormatEnum::ABGR8888).unwrap();
        let diamond_surface =
            Surface::from_data(&mut diamond_data, 24, 48, 96, PixelFormatEnum::ABGR8888).unwrap();
        let diamond_icon_surface = Surface::from_data(
            &mut diamond_icon_data,
            17,
            13,
            68,
            PixelFormatEnum::ABGR8888,
        )
        .unwrap();
        let exit_surface =
            Surface::from_data(&mut exit_data, 24, 48, 96, PixelFormatEnum::ABGR8888).unwrap();
        let exit_locked_surface =
            Surface::from_data(&mut exit_locked_data, 24, 48, 96, PixelFormatEnum::ABGR8888)
                .unwrap();
        let floor_green_surface =
            Surface::from_data(&mut floor_green_data, 24, 48, 96, PixelFormatEnum::ABGR8888)
                .unwrap();
        let floor_purple_surface = Surface::from_data(
            &mut floor_purple_data,
            24,
            48,
            96,
            PixelFormatEnum::ABGR8888,
        )
        .unwrap();
        let shovel_surface =
            Surface::from_data(&mut shovel_data, 24, 48, 96, PixelFormatEnum::ABGR8888).unwrap();
        let stone_surface =
            Surface::from_data(&mut stone_data, 24, 48, 96, PixelFormatEnum::ABGR8888).unwrap();
        let wall_surface =
            Surface::from_data(&mut wall_data, 24, 48, 96, PixelFormatEnum::ABGR8888).unwrap();

        Self {
            background: background_surface.as_texture(&tc).unwrap(),
            border: border_surface.as_texture(&tc).unwrap(),
            cadence_1: cadence_1_surface.as_texture(&tc).unwrap(),
            cadence_2: cadence_2_surface.as_texture(&tc).unwrap(),
            cadence_3: cadence_3_surface.as_texture(&tc).unwrap(),
            cadence_4: cadence_4_surface.as_texture(&tc).unwrap(),
            diamond: diamond_surface.as_texture(&tc).unwrap(),
            diamond_icon: diamond_icon_surface.as_texture(&tc).unwrap(),
            exit: exit_surface.as_texture(&tc).unwrap(),
            exit_locked: exit_locked_surface.as_texture(&tc).unwrap(),
            floor_green: floor_green_surface.as_texture(&tc).unwrap(),
            floor_purple: floor_purple_surface.as_texture(&tc).unwrap(),
            shovel: shovel_surface.as_texture(&tc).unwrap(),
            stone: stone_surface.as_texture(&tc).unwrap(),
            wall: wall_surface.as_texture(&tc).unwrap(),
        }
    }

    pub fn background(&self) -> &Texture<'lt> {
        &self.background
    }

    pub fn border(&self) -> &Texture<'lt> {
        &self.border
    }

    pub fn cadence_1(&self) -> &Texture<'lt> {
        &self.cadence_1
    }

    pub fn cadence_2(&self) -> &Texture<'lt> {
        &self.cadence_2
    }

    pub fn cadence_3(&self) -> &Texture<'lt> {
        &self.cadence_3
    }

    pub fn cadence_4(&self) -> &Texture<'lt> {
        &self.cadence_4
    }

    pub fn diamond(&self) -> &Texture<'lt> {
        &self.diamond
    }

    pub fn diamond_icon(&self) -> &Texture<'lt> {
        &self.diamond_icon
    }

    pub fn exit(&self) -> &Texture<'lt> {
        &self.exit
    }

    pub fn exit_locked(&self) -> &Texture<'lt> {
        &self.exit_locked
    }

    pub fn floor_green(&self) -> &Texture<'lt> {
        &self.floor_green
    }

    pub fn floor_purple(&self) -> &Texture<'lt> {
        &self.floor_purple
    }

    pub fn shovel(&self) -> &Texture<'lt> {
        &self.shovel
    }

    pub fn stone(&self) -> &Texture<'lt> {
        &self.stone
    }

    pub fn wall(&self) -> &Texture<'lt> {
        &self.wall
    }
}
