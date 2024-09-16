import { fail } from '@sveltejs/kit';
import type { PageServerLoad} from './$types';
import type { GameBackendDesc } from '$lib/stores/replayStore';

export const load = (({ params }) => {
  return {
    slug : params.slug,
    cache: new Map<number, GameBackendDesc>(),
  }
}) satisfies PageServerLoad;

