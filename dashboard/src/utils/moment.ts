import moment from "moment";

export const ago = (v: Date) => moment(v).fromNow();

export const show = (v: Date) => moment(v).format("LLLL");
