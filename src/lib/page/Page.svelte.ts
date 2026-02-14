import type { Uuid } from "$lib/types";
import { CircleQuestionMark, type IconProps } from "@lucide/svelte";
import type { Component } from "svelte";
import { v4 as uuidv4 } from 'uuid';

export type StringFactory<T> = (string | ((self?: StatefulPage<T>) => string))

export abstract class Page {
  abstract title: string;
  icon: Component<IconProps> = CircleQuestionMark;
  readonly id: Uuid = uuidv4() as Uuid;
  
  onOpen?(): void;
  
  onAdd?(): void;
  
  abstract getUrl(): string;
}

export abstract class StatefulPage<T> extends Page {
  data?: T = $state();
}