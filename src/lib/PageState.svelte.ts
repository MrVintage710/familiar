import { goto } from "$app/navigation";
import { v4 as uuidv4 } from 'uuid';
import type { Component } from "svelte";
import type { Uuid } from "./types";
import { House, Shield, Users, type IconProps } from "@lucide/svelte";


export const HOME_PAGE_UUID: Uuid = "28b87152-6893-4458-b4c2-e34cf864c2a7"
export const CHARACTER_PAGE_UUID : Uuid = "424bcb0e-a5ea-401a-9e8f-af581dad365e"

//==============================================================================================
//        Page State
//==============================================================================================

export type PageMap = {
  [key : Uuid] : Page
}

let currentPage: Uuid = $state(HOME_PAGE_UUID);

export const shortcuts : PageMap = $state({
  [HOME_PAGE_UUID]: {
    title: "Home",
    icon: House,
    url: "/",
    id: HOME_PAGE_UUID
  },
  [CHARACTER_PAGE_UUID]: {
    title: "Chracters",
    icon: Users,
    url: "/character",
    id: CHARACTER_PAGE_UUID
  }
})
const shortcutList = $derived(Object.values(shortcuts));

export const pages: PageMap = $state({
  '00940904-18f4-44c2-819e-0fbdb4f3e3ba': {
    title: "Test",
    icon: Shield,
    url: "/",
    id: '00940904-18f4-44c2-819e-0fbdb4f3e3ba'
  }
})
const pageList = $derived(Object.values(pages));

export function gotoAddPage(page: Page) {
  const uuid : Uuid = uuidv4() as Uuid;
  page.id = uuid;
  pages[uuid] = {...page};
  goto(page.url)
}

export function gotoPage(id? : Uuid) {
  const page = getPage(id);
  if (page?.id) {
    goto(page.url)
    currentPage = page.id
  }
}

export function getCurrentPage() {
  return getPage(currentPage)
}

export function removePage(id?: Uuid) {
  if(id) delete pages[id]
}

export function getPage(id?: Uuid) {
  if (!id) return undefined;
  return shortcuts[id] ?? pages[id] ?? undefined
}

export function getPageList(): Page[] {
  return pageList
}

export function getShortcutList(): Page[] {
  return shortcutList
}

//==============================================================================================
//        Page
//==============================================================================================

export interface Page {
  title: string;
  icon: Component<IconProps>;
  url: string;
  id?: Uuid
}
