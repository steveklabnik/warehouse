import DS from 'ember-data';
import ENV from "../config/environment";

export default DS.JSONAPIAdapter.extend({
//    namespace: 'v1',
    host: ENV.APP.API_HOST,
});
