// Copyright 2020 Revcore Technologies Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::Error;
use proj::{AloeVeraProject, Jsonable};
use util::fat;

/// Arguments for the initial create project command
pub struct CreateProjectArgs {
	pub id: Option<String>,
	pub output_file: String,
}

pub fn create_project(args: &CreateProjectArgs) -> Result<(), Error> {
	let id = match &args.id {
		Some(l) => l,
		None => "My Project",
	};
	info!("Creating new project file at: {}", args.output_file);

	let proj = AloeVeraProject::new(id);
	let json = proj.to_json()?;
	crate::cmd::common::output_to_file(&args.output_file, &json.as_bytes(), &None)?;

	Ok(())
}

/// Arguments for the initial create project command
pub struct CreateSDImageArgs {
	pub output_file: String,
}

pub fn create_sd_image(args: &CreateSDImageArgs) -> Result<(), Error> {
	info!(
		"Creating new SDCard (FAT32 FS) image at: {}",
		args.output_file
	);

	fat::create_fat_image(&args.output_file)?;

	Ok(())
}
