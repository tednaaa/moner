// NOTE: This file is based on primevue tailwind Aura preset
// they at this time do not support typescript, in v4 they will
// see https://github.com/primefaces/primevue-tailwind/issues/66
// @ts-nocheck

import { global } from "./global";
import autocomplete from "./preset/autocomplete";
import calendar from "./preset/calendar";
import cascadeselect from "./preset/cascadeselect";
import checkbox from "./preset/checkbox";
import chips from "./preset/chips";
import colorpicker from "./preset/colorpicker";
import dropdown from "./preset/dropdown";
import floatlabel from "./preset/floatlabel";
import iconfield from "./preset/iconfield";
import inputgroup from "./preset/inputgroup";
import inputotp from "./preset/inputotp";
import inputgroupaddon from "./preset/inputgroupaddon";
import inputmask from "./preset/inputmask";
import inputnumber from "./preset/inputnumber";
import inputswitch from "./preset/inputswitch";
import inputtext from "./preset/inputtext";
import knob from "./preset/knob";
import listbox from "./preset/listbox";
import multiselect from "./preset/multiselect";
import password from "./preset/password";
import radiobutton from "./preset/radiobutton";
import rating from "./preset/rating";
import selectbutton from "./preset/selectbutton";
import slider from "./preset/slider";
import textarea from "./preset/textarea";
import togglebutton from "./preset/togglebutton";
import treeselect from "./preset/treeselect";
import tristatecheckbox from "./preset/tristatecheckbox";
import button from "./preset/button";
import speeddial from "./preset/speeddial";
import splitbutton from "./preset/splitbutton";
import datatable from "./preset/datatable";
import dataview from "./preset/dataview";
import orderlist from "./preset/orderlist";
import organizationchart from "./preset/organizationchart";
import paginator from "./preset/paginator";
import picklist from "./preset/picklist";
import tree from "./preset/tree";
import treetable from "./preset/treetable";
import timeline from "./preset/timeline";
import accordion from "./preset/accordion";
import card from "./preset/card";
import deferred from "./preset/deferred";
import divider from "./preset/divider";
import fieldset from "./preset/fieldset";
import panel from "./preset/panel";
import scrollpanel from "./preset/scrollpanel";
import splitter from "./preset/splitter";
import stepper from "./preset/stepper";
import tabview from "./preset/tabview";
import toolbar from "./preset/toolbar";
import confirmpopup from "./preset/confirmpopup";
import dialog from "./preset/dialog";
import overlaypanel from "./preset/overlaypanel";
import sidebar from "./preset/sidebar";
import tooltip from "./preset/tooltip";
import fileupload from "./preset/fileupload";
import breadcrumb from "./preset/breadcrumb";
import contextmenu from "./preset/contextmenu";
import dock from "./preset/dock";
import menu from "./preset/menu";
import menubar from "./preset/menubar";
import megamenu from "./preset/megamenu";
import panelmenu from "./preset/panelmenu";
import steps from "./preset/steps";
import tabmenu from "./preset/tabmenu";
import tieredmenu from "./preset/tieredmenu";
import message from "./preset/message";
import inlinemessage from "./preset/inlinemessage";
import toast from "./preset/toast";
import carousel from "./preset/carousel";
import galleria from "./preset/galleria";
import image from "./preset/image";
import avatar from "./preset/avatar";
import badge from "./preset/badge";
import badgedirective from "./preset/badgedirective";
import blockui from "./preset/blockui";
import chip from "./preset/chip";
import inplace from "./preset/inplace";
import metergroup from "./preset/metergroup";
import scrolltop from "./preset/scrolltop";
import skeleton from "./preset/skeleton";
import progressbar from "./preset/progressbar";
import progressspinner from "./preset/progressspinner";
import ripple from "./preset/ripple";
import tag from "./preset/tag";
import terminal from "./preset/terminal";

export const pt = {
  global,
  directives: {
    tooltip,
    badgedirective,
    ripple,
  },
  autocomplete,
  calendar,
  cascadeselect,
  checkbox,
  chips,
  colorpicker,
  dropdown,
  floatlabel,
  iconfield,
  inputgroup,
  inputotp,
  inputgroupaddon,
  inputmask,
  inputnumber,
  inputswitch,
  inputtext,
  knob,
  listbox,
  multiselect,
  password,
  radiobutton,
  rating,
  selectbutton,
  slider,
  textarea,
  togglebutton,
  treeselect,
  tristatecheckbox,
  button,
  speeddial,
  splitbutton,
  datatable,
  dataview,
  orderlist,
  organizationchart,
  paginator,
  picklist,
  tree,
  treetable,
  timeline,
  accordion,
  card,
  deferred,
  divider,
  fieldset,
  panel,
  scrollpanel,
  splitter,
  stepper,
  tabview,
  toolbar,
  confirmpopup,
  dialog,
  overlaypanel,
  sidebar,
  fileupload,
  breadcrumb,
  contextmenu,
  dock,
  menu,
  menubar,
  megamenu,
  panelmenu,
  steps,
  tabmenu,
  tieredmenu,
  message,
  inlinemessage,
  toast,
  carousel,
  galleria,
  image,
  avatar,
  badge,
  blockui,
  chip,
  inplace,
  metergroup,
  scrolltop,
  skeleton,
  progressbar,
  progressspinner,
  tag,
  terminal,
};
