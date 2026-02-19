import { goto } from "$app/navigation";
import type { Uuid } from "../types";
import CharacterSelectionPage, { CHARACTER_PAGE_UUID } from "./CharacterSelectionPage.svelte";
import type { Page, PageData } from "./Page.svelte";

import CreateCharacterPage from "./CreateCharacterPage.svelte";


export const HOME_PAGE_UUID: Uuid = "28b87152-6893-4458-b4c2-e34cf864c2a7"

//==============================================================================================
//        Page State
//==============================================================================================

type PageProxy = {
  id: Uuid,
  pageType: string,
  state: any | null
}

class PageStateClass {
  public pages: Page[] = $state([]);
  public shortcuts: Page[] = $state([
    new CharacterSelectionPage()
  ]);
  private currentPage: number = $state(0);
  public currentPageId: Uuid | null = $derived(this.indexPage(this.currentPage)?.id ?? null)
  
  addPage(page: Page, shouldSave: boolean = true) {
    this.pages.push(page)
    if (shouldSave) this.savePages();
  }
  
  removePage(id: Uuid, shouldSave: boolean = true) {
    let index = this.getPageIndex(id);
    this.pages = this.pages.filter(page => id !== page.id)
    if (this.currentPage >= this.shortcuts.length + this.pages.length) {
      this.currentPage = (this.shortcuts.length + this.pages.length - 1)
      this.gotoIndex(this.currentPage)
    }
    if (shouldSave) {
      this.savePages();
      sessionStorage.setItem("currentPage", String($state.snapshot(this.currentPage)))
    }
  }
  
  addShortcut(page: Page) {
    this.shortcuts.push(page)
  }
  
  getPage(uuid?: Uuid): Page | undefined {
    if (!uuid) return undefined;
    return this.shortcuts.find(page => page.id === uuid) ?? this.pages.find(page => page.id === uuid);
  }
  
  getPageIndex(uuid?: Uuid): number {
    if (!uuid) return -1;
    const shortcutIndex = this.shortcuts.findIndex(page => page.id === uuid);
    const pageIndex = this.pages.findIndex(page => page.id === uuid)
    return shortcutIndex < 0 ? (pageIndex < 0 ? -1 : pageIndex + this.shortcuts.length) : shortcutIndex;
  }
  
  indexPage(index: number): Page | undefined {
    if (index < 0) return undefined;
    return this.shortcuts[index] ?? this.pages[index - this.shortcuts.length]
  }
  
  getCurrentPage(): Page | undefined {
    return this.shortcuts[this.currentPage] ?? this.pages[this.currentPage - this.shortcuts.length]
  }
  
  isPageSeletected(uuid: Uuid): boolean {
    return this.getPageIndex(uuid) === this.currentPage
  }
  
  gotoPage(uuid: Uuid) {
    const index = this.getPageIndex(uuid);
    const page = this.indexPage(index);
    if (index >= 0 && page) {
      goto(page.getUrl());
      this.currentPage = index;
      page.onOpen?.()
      sessionStorage.setItem("currentPage", String($state.snapshot(this.currentPage)))
    }
  }
  
  gotoIndex(index: number) {
    const page = this.indexPage(index);
    if (index >= 0 && page) {
      goto(page.getUrl());
      this.currentPage = index;
      page.onOpen?.()
      sessionStorage.setItem("currentPage", String($state.snapshot(this.currentPage)))
    }
  }
  
  init() {
    this.loadPages()
    this.shortcuts.forEach(page => page.onAdd?.())
  }
  
  savePages() {
    const storedPages = this.pages
      .filter(page => !!page)
      .map(page => ({
        id: page.id,
        pageType: page.constructor.name,
        state: page.onSave?.() ?? null
      }));
    sessionStorage.setItem("pages", JSON.stringify(storedPages))
  }
  
  loadPages() {
    const pageStorage: PageProxy[] = JSON.parse(sessionStorage.getItem("pages") ?? "[]")
    this.currentPage = Number(sessionStorage.getItem("currentPage") ?? 0);
    this.pages = pageStorage.map(page => {
      let pageInstance : Page = eval("new " + page.pageType + "(page.state)");
      pageInstance.id = page.id;
      if(page.state != null) pageInstance.onLoad?.(page.state)
      return pageInstance
    })
  }
}

const PageState = new PageStateClass()

export default PageState;