import type { ToastPassThroughOptions } from 'primevue/toast';
import styles from './toast.module.scss';

export const toastStyles: ToastPassThroughOptions = {
  container: (({ props: { message } }) => (message?.severity && {
    class: [styles.toast, styles[message.severity]],
  })),
  content: styles.content,
  message: styles.message,
  text: styles.text,
  detail: styles.detail,
  icon: styles.icon,
  closeButton: styles.closeButton,
  closeIcon: styles.closeIcon,
}
