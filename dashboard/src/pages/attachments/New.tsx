import { Button, Icon, message, Upload } from 'antd'
import { UploadChangeParam } from 'antd/lib/upload'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { connect } from 'react-redux'
import { RouteComponentProps, withRouter } from 'react-router'

import { Authorized } from '../../components/authorized'
import Layout from '../../components/form/Layout'
import { IApplicationState } from '../../reducers'
import { get as getToken } from '../../utils/token'

class Widget extends React.Component<RouteComponentProps<any> & InjectedIntlProps> {
    public static propTypes: React.ValidationMap<any> = {
        intl: intlShape.isRequired,
    }

    public handleUpload = (info: UploadChangeParam) => {
        const { formatMessage } = this.props.intl
        if (info.file.status !== 'uploading') {
            message.info(formatMessage(
                { id: 'nut.attachments.new.uploading' },
                { title: info.file.name },
            ))
        }
        if (info.file.status === 'done') {
            message.success(formatMessage(
                { id: 'nut.attachments.new.done' },
                { title: info.file.name },
            ))
        } else if (info.file.status === 'error') {
            message.error(formatMessage(
                { id: 'nut.attachments.new.error' },
                { title: info.file.name },
            ))
        }
    }

    public render() {
        return (<Authorized>
            <Layout title={{ id: 'nut.attachments.new.title' }}>
                <Upload
                    name='file'
                    action='/api/attachments'
                    multiple={true}
                    headers={{
                        authorization: `Bearer ${getToken()}`,
                    }}
                    onChange={this.handleUpload}                >
                    <Button>
                        <Icon type="upload" /> <FormattedMessage id="buttons.upload" />
                    </Button>
                </Upload>
            </Layout>
        </Authorized >)
    }
}

const mapStateToProps = ({ }: IApplicationState) => ({})

const mapDispatchToProps = () => ({})

export default withRouter(connect(
    mapStateToProps,
    mapDispatchToProps
)(injectIntl(Widget)))
