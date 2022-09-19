use std::ptr::null;
use std::u32;
use gtk::ffi::{GtkCellRenderer, GtkScrolledWindow, GtkTreeView, GtkTreeViewColumn};
use gtk::gdk::keys::constants::{equal, f};
use gtk::gio::ffi::{g_drive_get_type, GDrive};
use gtk::glib::{StaticType, ToValue, Type, Value};
use gtk::{false_, gio, ListStore, TreeIter};
use gtk::pango::ffi::PANGO_WEIGHT_BOLD;
use gtk::prelude::{GtkListStoreExtManual, TreeStoreExtManual};
use gtk::traits::{CellRendererExt, CellRendererTextExt, TreeStoreExt, TreeViewColumnExt, TreeViewExt, WidgetExt};
use crate::nemo_places_sidebar::PLACES_SIDEBAR::{PLACES_SIDEBAR_COLUMN_BOOKMARK, PLACES_SIDEBAR_COLUMN_DF_PERCENT, PLACES_SIDEBAR_COLUMN_DRIVE, PLACES_SIDEBAR_COLUMN_EJECT, PLACES_SIDEBAR_COLUMN_EJECT_ICON, PLACES_SIDEBAR_COLUMN_GICON, PLACES_SIDEBAR_COLUMN_INDEX, PLACES_SIDEBAR_COLUMN_MOUNT, PLACES_SIDEBAR_COLUMN_NAME, PLACES_SIDEBAR_COLUMN_NO_EJECT, PLACES_SIDEBAR_COLUMN_ROW_TYPE, PLACES_SIDEBAR_COLUMN_SECTION_TYPE, PLACES_SIDEBAR_COLUMN_SHOW_DF, PLACES_SIDEBAR_COLUMN_TOOLTIP, PLACES_SIDEBAR_COLUMN_URI, PLACES_SIDEBAR_COLUMN_VOLUME};
use crate::nemo_places_sidebar::PlaceType::PLACES_BOOKMARK;

struct NemoPlacesSidebar {
    parent : GtkScrolledWindow,
    tree_view : GtkTreeView,
    eject_icon_cel_renderer : GtkCellRenderer,
    uri : char,
}


fn append_column(tree: &gtk::TreeView, id: i32) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}

pub trait TreePlaces {
    fn add_place(
        &self,
        place_type : PlaceType,
        section_type : SectionType,
        name : String,
        icon_name : String,
        uri : String,
        drive : gio::Drive,
        volumne : gio::Volume,
        mount : gio::Mount,
        index : i32,
        tooltip : String,
        df_percent : i32,
        show_df_percent : bool,
        cat_iter : TreeIter
    );
}

#[derive(Copy, Clone,PartialEq)]
enum SectionType {
    SECTION_COMPUTER,
    SECTION_XDG_BOOKMARKS,
    SECTION_BOOKMARKS,
    SECTION_DEVICES,
    SECTION_NETWORK,
}

impl ToValue for SectionType {
    fn to_value(&self) -> Value {
        i32::to_value(&(*self as i32))
    }

    fn value_type(&self) -> Type {
        i32::static_type()
    }

}

#[derive(Copy, Clone,PartialEq)]
enum PlaceType {
    PLACES_BUILT_IN,
    PLACES_XDG_DIR,
    PLACES_MOUNTED_VOLUME,
    PLACES_BOOKMARK,
    PLACES_HEADING,
}

impl ToValue for PlaceType {
    fn to_value(&self) -> Value {
        i32::to_value(&(*self as i32))
    }

    fn value_type(&self) -> Type {
        i32::static_type()
    }

}

enum PLACES_SIDEBAR {
    PLACES_SIDEBAR_COLUMN_ROW_TYPE,
    PLACES_SIDEBAR_COLUMN_URI,
    PLACES_SIDEBAR_COLUMN_DRIVE,
    PLACES_SIDEBAR_COLUMN_VOLUME,
    PLACES_SIDEBAR_COLUMN_MOUNT,
    PLACES_SIDEBAR_COLUMN_NAME,
    PLACES_SIDEBAR_COLUMN_GICON,
    PLACES_SIDEBAR_COLUMN_INDEX,
    PLACES_SIDEBAR_COLUMN_EJECT,
    PLACES_SIDEBAR_COLUMN_NO_EJECT,
    PLACES_SIDEBAR_COLUMN_BOOKMARK,
    PLACES_SIDEBAR_COLUMN_TOOLTIP,
    PLACES_SIDEBAR_COLUMN_EJECT_ICON,
    PLACES_SIDEBAR_COLUMN_SECTION_TYPE,
    PLACES_SIDEBAR_COLUMN_HEADING_TEXT,
    PLACES_SIDEBAR_COLUMN_DF_PERCENT,
    PLACES_SIDEBAR_COLUMN_SHOW_DF,

    PLACES_SIDEBAR_COLUMN_COUNT
}

impl TreePlaces for gtk::TreeStore {
    fn add_place(
        &self,
        place_type : PlaceType,
        section_type : SectionType,
        name : String,
        icon_name : String,
        uri : String,
        drive : gio::Drive,
        volumne : gio::Volume,
        mount : gio::Mount,
        index : i32,
        tooltip : String,
        df_percent : i32,
        show_df_percent : bool,
        cat_iter : TreeIter
    ) {

        let gicon = gio::ThemedIcon::new(&icon_name);
        let bookmark = place_type != PLACES_BOOKMARK;


        let iter = self.append(None);
        self.set(&iter, &[
            (PLACES_SIDEBAR_COLUMN_GICON as u32, &gicon),
            (PLACES_SIDEBAR_COLUMN_NAME as u32, &name),
            (PLACES_SIDEBAR_COLUMN_URI as u32, &uri),
            (PLACES_SIDEBAR_COLUMN_DRIVE as u32, &drive),
            (PLACES_SIDEBAR_COLUMN_VOLUME as u32, &volumne),
            (PLACES_SIDEBAR_COLUMN_MOUNT as u32, &mount),
            (PLACES_SIDEBAR_COLUMN_ROW_TYPE as u32, &place_type),
            (PLACES_SIDEBAR_COLUMN_INDEX as u32, &index),
            (PLACES_SIDEBAR_COLUMN_EJECT as u32, &true),
            (PLACES_SIDEBAR_COLUMN_NO_EJECT as u32, &false),
            (PLACES_SIDEBAR_COLUMN_BOOKMARK as u32, &bookmark),
            (PLACES_SIDEBAR_COLUMN_TOOLTIP as u32, &tooltip),
            (PLACES_SIDEBAR_COLUMN_EJECT_ICON as u32, &"media-eject-symbolic"),
            (PLACES_SIDEBAR_COLUMN_SECTION_TYPE as u32, &section_type),
            (PLACES_SIDEBAR_COLUMN_DF_PERCENT as u32, &df_percent),
            (PLACES_SIDEBAR_COLUMN_SHOW_DF as u32, &show_df_percent
            ),
        ])

    }
}

fn nemo_shortcuts_model_new() -> gtk::TreeStore
{
    gtk::TreeStore::new(&[
        i32::static_type(),
        String::static_type(),
        gio::Drive::static_type(),
        gio::Volume::static_type(),
        gio::Mount::static_type(),
        String::static_type(),
        gio::Icon::static_type(),
        i32::static_type(),
        bool::static_type(),
        bool::static_type(),
        bool::static_type(),
        String::static_type(),
        String::static_type(),
        i32::static_type(),
        String::static_type(),
        i32::static_type(),
        bool::static_type(),
    ])
}


pub fn nemo_places_sidebar_init() -> gtk::TreeView
{
    let tree_view : gtk::TreeView = gtk::TreeView::new();
    let expander_pad_col : gtk::TreeViewColumn = gtk::TreeViewColumn::new();
    let col : gtk::TreeViewColumn = gtk::TreeViewColumn::new();
    let mut cell: gtk::CellRendererText = gtk::CellRendererText::new();//G_TYPE_CHAR
    let store : gtk::TreeStore = nemo_shortcuts_model_new();
    let test_store: ListStore =  ListStore::new(&[u32::static_type(), String::static_type()]);

    append_column(&tree_view, 0);
    append_column(&tree_view, 1);


    let entries = &["Michel", "Sara", "Liam", "Zelda", "Neo", "Octopus master"];

    for (i, entry) in entries.iter().enumerate() {
        test_store.insert_with_values(None, &[(0, &1),(1, &entry)]);
        // store.insert_with_values(None, &[0, 1], &[&(i as u32 + 1), &entry]);
    }

    let store_filter : gtk::TreeModelFilter = gtk::TreeModelFilter::new(&store, None);



    col.pack_start(&cell, false);

    cell = gtk::CellRendererText::new();
    col.pack_start(&cell, false);
    col.set_attributes(&cell, &[("text", 15)]);

    cell.set_weight(PANGO_WEIGHT_BOLD);
    cell.set_weight_set(true);
    cell.set_ypad(0);
    cell.set_xpad(0);

    // col.set_cell_data_func(cell,heading_cell_renderer_func, side )


    expander_pad_col.set_sizing(gtk::TreeViewColumnSizing::Fixed);
    expander_pad_col.set_fixed_width(4);
    tree_view.append_column(&expander_pad_col);
    tree_view.append_column(&col);




    tree_view.set_model(Some(&store_filter));
    tree_view.show_all();


    tree_view

}


fn heading_cell_renderer_func(cell : &gtk::CellRenderer)
{
    cell.set_visible(true);
}