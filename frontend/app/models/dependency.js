import DS from 'ember-data';

export default DS.Model.extend({
  name: DS.attr('string'),
  requirement: DS.attr('string'),
  optional: DS.attr('boolean'),
  versions: DS.belongsTo('version'),
});
