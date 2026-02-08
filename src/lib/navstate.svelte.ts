//==============================================================================================
//        Page
//==============================================================================================

import { CircleQuestionMark, type IconProps } from "@lucide/svelte";
import type { Component } from "svelte";

export class Page<T> {
  data : T = $state<T>({} as T);
  title : string = "";
  icon : Component<IconProps> = CircleQuestionMark;
  
  constructor(title: string, data: T, icon?: Component<IconProps>) {
    this.title = title
    this.data = data;
    if(icon) this.icon = icon
  }
}