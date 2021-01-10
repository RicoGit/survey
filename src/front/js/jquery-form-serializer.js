(function($) {
	$.fn.serializeObject = function() {

		var self = this,
			json = {},
			push_counters = {},
			patterns = {
				"validate": /^[a-zA-Z][a-zA-Z0-9_]*(?:\[(?:\d*|[a-zA-Z0-9_]+)\])*$/,
				"key": /[a-zA-Z0-9_]+|(?=\[\])/g,
				"push": /^$/,
				"fixed": /^\d+$/,
				"named": /^[a-zA-Z0-9_]+$/
			};

		this.pair = function(base, key, value) {
			base[key] = value;
			return base;
		};

		this.push_counter = function(key) {
			if (push_counters[key] === undefined) {
				push_counters[key] = 0;
			}
			return push_counters[key]++;
		};

		$.each($(this).serializeArray(), function(index, item) {

			// skip invalid keys
			var name = item.name;
			if (!patterns.validate.test(name)) {
				return;
			}

			var key;
			var keys = name.match(patterns.key);
			var value = item.value;
			var reverse_key = name;

			while ((key = keys.pop()) !== undefined) {

				// adjust reverse_key
				reverse_key = reverse_key.replace(new RegExp("\\[" + key + "\\]$"), '');

				// push
				if (key.match(patterns.push)) {
					value = self.pair([], self.push_counter(reverse_key), value);
				}

				// fixed
				else if (key.match(patterns.fixed)) {
					value = self.pair([], key, value);
				}

				// named
				else if (key.match(patterns.named)) {
					value = self.pair({}, key, value);
				}
			}

			json = $.extend(true, json, value);
		});

		return json;
	};
})(jQuery);