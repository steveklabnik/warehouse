import Ember from 'ember';
import pagedArray from 'ember-cli-pagination/computed/paged-array';

export default Ember.Component.extend({
  perPage: 20,

  pagedContent: pagedArray('crates', {
    pageBinding: 'page',
    perPageBinding: 'perPage'
  }),
});
