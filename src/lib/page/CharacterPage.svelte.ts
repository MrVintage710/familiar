import { User, UserIcon, type IconProps } from "@lucide/svelte";
import type { Component } from "svelte";
import { Page, StatefulPage } from "./Page.svelte";
import type { Uuid } from "$lib/types";

export type CharacterPageState = {
  ruleset: string
}

export default class CharacterPage extends Page {
  title: string;
  ruleset: string;
  
  constructor(ruleset: string) {
    super();
    this.icon = User;
    this.title = "Character for " + ruleset;
    this.ruleset = ruleset
  }
  
  getUrl(): string {
    return `/character/${this.ruleset}`
  }
}