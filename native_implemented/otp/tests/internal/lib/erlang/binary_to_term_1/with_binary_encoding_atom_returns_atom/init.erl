-module(init).
-export([start/0]).
-import(erlang, [binary_to_term/1, display/1]).

start() ->
  display(binary_to_term(<<131, 100, 0, 4, 97, 116, 111, 109>>)).
