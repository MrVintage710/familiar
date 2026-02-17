import type { Uuid } from "$lib/types";
import { CircleQuestionMark, type IconProps } from "@lucide/svelte";
import type { Component } from "svelte";
import { v4 as uuidv4 } from 'uuid';

export abstract class Page {
  abstract title: string;
  icon: Component<IconProps> = CircleQuestionMark;
  id: Uuid = uuidv4() as Uuid;
  
  onOpen?(): void;
  
  onAdd?(): void;
  
  abstract getUrl(): string;
  
  onSave?(): any;
  
  onLoad?(data : any): void;
}

export interface PageData<T> {
  data: T
}