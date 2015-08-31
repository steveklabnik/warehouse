import Ember from 'ember';
import DS from 'ember-data';

const {
  Model,
  hasMany
} = DS;

const {
  computed: {
    alias,
    sort
  }
} = Ember;

export default Model.extend({
  versions: hasMany('version'),

  versionSorting: ['name'],
  sortedVersions: sort('versions', 'versionSorting'),

  currentVersion: alias('sortedVersions.lastObject'),
});
