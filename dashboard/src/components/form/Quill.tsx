import { Form } from 'antd'
import * as React from 'react'
import { FormattedMessage } from 'react-intl'
import ReactQuill from 'react-quill'

import { formItemLayout } from '.'

const FormItem = Form.Item

interface IProps {
  value: string,
  onChange: (content: string) => void,
}

class Widget extends React.Component<IProps> {
  public render() {
    const { value, onChange } = this.props
    const modules = {
      clipboard: {
        // toggle to add extra line breaks when pasting HTML:
        matchVisual: false
      },
      toolbar: [
        [
          {
            'font': []
          }
        ],
        [
          {
            size: []
          }
        ],
        [
          'bold', 'italic', 'underline', 'strike', 'blockquote'
        ],
        [
          {
            'list': 'ordered'
          }, {
            'list': 'bullet'
          }, {
            'indent': '-1'
          }, {
            'indent': '+1'
          }
        ],
        [
          'link', 'image', 'video'
        ],
        ['clean']
      ],
    }

    const formats = [
      'header',
      'font',
      'size',
      'bold',
      'italic',
      'underline',
      'strike',
      'blockquote',
      'list',
      'bullet',
      'indent',
      'link',
      'image',
      'video'
    ]
    return (<FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.body" />}>
      <ReactQuill modules={modules} formats={formats} value={value} onChange={onChange} theme="snow" />
    </FormItem>)
  }
}


export default Widget
