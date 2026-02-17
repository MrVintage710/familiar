import { goto } from "$app/navigation";
import type { Uuid } from "../types";
import CharacterSelectionPage, { CHARACTER_PAGE_UUID } from "./CharacterSelectionPage.svelte";
import type { Page, PageData } from "./Page.svelte";

import CreateCharacterPage from "./CreateCharacterPage.svelte";


export const HOME_PAGE_UUID: Uuid = "28b87152-6893-4458-b4c2-e34cf864c2a7"

//==============================================================================================
//        Page State
//==============================================================================================

let pages: Page[] = $state([]);
const shortcuts: Page[] = $state([
  new CharacterSelectionPage()
]);

let currentPage: Uuid = $state(CHARACTER_PAGE_UUID);

export function addPage(page : Page) {
  pages.push(page)
}

export function addShortcut(page: Page) {
  shortcuts.push(page)
}

export function getPageList(): Page[] {
  return pages;
}

export function getShortcutList(): Page[] {
  return shortcuts;
}

export function getPage(uuid?: Uuid): Page | undefined {
  if (!uuid) return undefined;
  return shortcuts.find(page => page.id === uuid) ?? pages.find(page => page.id === uuid);
}

export function getCurrentPage(): Page | undefined {
  return getPage(currentPage)
}

export function gotoPage(uuid: Uuid, shouldSave : boolean = true) {
  const page = getPage(uuid);
  if (page) {
    // console.log(JSON.stringify(page), String($state.snapshot(page).icon))
    goto(page.getUrl());
    currentPage = page.id;
    page.onOpen?.()
    sessionStorage.setItem("currentPage", $state.snapshot(currentPage))
    if(shouldSave) savePages()
  }
}

export function initNavstate() {
  loadPages()
  shortcuts.forEach(page => page.onAdd?.())
  gotoPage(currentPage);
}

type PageProxy = {
  id: Uuid,
  pageType: string,
  state: any | null
}

function savePages() {
  const storedPages = pages.map(page => ({
    id: page.id,
    pageType: page.constructor.name,
    state: page.onSave?.() ?? null
  }));
  sessionStorage.setItem("pages", JSON.stringify(storedPages))
}

function loadPages() {
  const pageStorage: PageProxy[] = JSON.parse(sessionStorage.getItem("pages") ?? "[]")
  currentPage = sessionStorage.getItem("currentPage") as Uuid ?? CHARACTER_PAGE_UUID;
  pages = pageStorage.map(page => {
    let pageInstance : Page = eval("new " + page.pageType + "(page.state)");
    pageInstance.id = page.id;
    if(page.state != null) pageInstance.onLoad?.(page.state)
    return pageInstance
  })
}