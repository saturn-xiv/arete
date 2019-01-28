import { Form } from 'antd'
import * as React from 'react'
import { ColorResult, SketchPicker } from 'react-color'
import { FormattedMessage } from 'react-intl'

import { formItemLayout } from '.'

const FormItem = Form.Item

interface IProps {
    defaultValue: string,
    onChange: (v: string) => void,
}

class Widget extends React.Component<IProps> {
    public handleChange = (e: ColorResult) => {
        this.props.onChange(e.hex)
    }
    public render() {
        return (<FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.color" />}>
            <SketchPicker color={this.props.defaultValue} onChangeComplete={this.handleChange} />
        </FormItem>)
    }
}

export default Widget
