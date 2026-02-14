import { goto } from "$app/navigation";
import { page } from "$app/state";
import type { Uuid } from "../types";
import CharacterSelectionPage, { CHARACTER_PAGE_UUID } from "./CharacterSelectionPage.svelte";
import type { Page } from "./Page.svelte";


export const HOME_PAGE_UUID: Uuid = "28b87152-6893-4458-b4c2-e34cf864c2a7"

//==============================================================================================
//        Page State
//==============================================================================================

const pages: Page[] = $state([]);
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

export function gotoPage(uuid: Uuid) {
  const page = getPage(uuid);
  console.log("gotoPage", uuid, page, pages)
  if (page) {
    goto(page.getUrl());
    currentPage = page.id;
    page.onOpen?.()
  }
}

export function initNavstate() {
  shortcuts.forEach(page => page.onAdd?.())
  gotoPage(currentPage);
}