#!/bin/bash

# Define the new directory structure based on the provided screens and sections
declare -A new_structure=(
	["application_shells"]="stacked_layouts sidebar_layouts multi_column_layouts"
	["page_examples"]="home_screens detail_screens settings_screens"
	["headings"]="page_headings card_headings section_headings"
	["data_display"]="description_lists stats calendars"
	["lists"]="stacked_lists tables grid_lists feeds"
	["forms"]="form_layouts input_groups select_menus sign_in_and_registration textareas radio_groups checkboxes toggles action_panels comboboxes"
	["feedback"]="alerts empty_states"
	["navigation"]="navbars pagination tabs vertical_navigation sidebar_navigation breadcrumbs progress_bars command_palettes"
	["overlays"]="dialogs slide_overs notifications"
	["elements"]="avatars badges dropdowns buttons button_groups"
	["layout"]="containers panels list_containers media_objects dividers"
)

# Base directory
base_dir="frontend/webui/src/pages/blocks"

# Remove all existing directories that are not in the new structure
find "${base_dir}" -mindepth 1 -maxdepth 1 -type d | while read -r dir; do
	dir_name=$(basename "${dir}")
	if ! grep -q "${dir_name}" <<<"${new_structure[*]}"; then
		rm -rf "${dir}"
	fi
done

# Create new directories and touch mod.rs files
for section in "${!new_structure[@]}"; do
	for dir in ${new_structure[${section}]}; do
		mkdir -p "${base_dir}/${dir}"
		touch "${base_dir}/${dir}/mod.rs"
	done
done

# Move existing directories to the new structure if they are reused
for section in "${!new_structure[@]}"; do
	for dir in ${new_structure[${section}]}; do
		if [[ -d "${base_dir}/${dir}" ]]; then
			mv "${base_dir}/${dir}" "${base_dir}/${section}/"
		fi
	done
done

# Documentation
: <<'END_DOC'
This script refactors the directory layout of the project based on the provided screens and sections. It performs the following steps:

1. Defines the new directory structure using an associative array where keys are section names and values are space-separated lists of directory names.
2. Sets the base directory to "frontend/webui/src/pages/blocks".
3. Removes all existing directories in the base directory that are not part of the new structure.
4. Creates new directories as specified in the new structure and touches a "mod.rs" file in each of them.
5. Moves existing directories to their new locations if they are reused in the new structure.

The script ensures that the directory layout is updated according to the new structure while preserving any directories that are reused.
END_DOC
