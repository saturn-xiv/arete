import React, { Component } from "react";

import Quill from "react-quill";
import { Sources, Delta } from "quill";

interface IProps {
  value?: string | Delta;

  onChange?: (content: string, delta: Delta, source: Sources) => void;
}
interface IState {}

class Widget extends Component<IProps, IState> {
  render() {
    const { value, onChange } = this.props;
    const modules = {
      toolbar: [
        [
          {
            font: []
          }
        ],
        [
          {
            size: []
          }
        ],
        ["bold", "italic", "underline", "strike", "blockquote"],
        [
          {
            list: "ordered"
          },
          {
            list: "bullet"
          },
          {
            indent: "-1"
          },
          {
            indent: "+1"
          }
        ],
        ["link", "image", "video"],
        ["clean"]
      ],
      clipboard: {
        // toggle to add extra line breaks when pasting HTML:
        matchVisual: false
      }
    };

    const formats = [
      "header",
      "font",
      "size",
      "bold",
      "italic",
      "underline",
      "strike",
      "blockquote",
      "list",
      "bullet",
      "indent",
      "link",
      "image",
      "video"
    ];
    return (
      <Quill
        modules={modules}
        formats={formats}
        value={value}
        onChange={onChange}
        theme="snow"
      />
    );
  }
}

export default Widget;
