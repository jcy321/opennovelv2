import type { PageLoad } from './$types';
import { bookActions, messageActions } from '$lib/stores/actions';
import { activeBookId } from '$lib/stores/index';
import { browser } from '$app/environment';

export const load: PageLoad = async ({ params }) => {
  const bookId = params.id;
  
  // 设置当前活跃书籍
  if (browser) {
    activeBookId.set(bookId);
    
    // 加载消息
    await messageActions.loadMessages(bookId);
  }
  
  return {
    bookId,
  };
};