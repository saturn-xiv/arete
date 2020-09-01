import moment from "moment-timezone";

export const ago = (v: Date): string => moment.utc(v).fromNow();

export const show = (v: Date): string => moment.utc(v).local().format("LLLL");

export const is_after = (v: Date, seconds: number): boolean =>
  moment().isAfter(moment.utc(v).add(seconds, "seconds"));
