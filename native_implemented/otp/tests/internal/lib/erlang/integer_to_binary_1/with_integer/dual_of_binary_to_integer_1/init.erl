-module(init).
-export([start/0]).
-import(erlang, [display/1, integer_to_binary/1]).

start() ->
  dual(-1),
  dual(0),
  dual(1).

dual(Integer) ->
  display(Integer == binary_to_integer(integer_to_binary(Integer))).


